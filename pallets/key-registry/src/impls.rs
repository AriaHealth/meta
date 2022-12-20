use super::pallet::*;
use crate::constants::{FETCH_TIMEOUT_PERIOD, HTTP_REMOTE_REQUEST, LOCK_BLOCK_EXPIRATION, LOCK_TIMEOUT_EXPIRATION, NUM_VEC_LEN};
use crate::types::{HackerNewsInfo, Payload};
use core::convert::TryInto;
use frame_system::offchain::{SendSignedTransaction, SendUnsignedTransaction, Signer, SubmitTransaction};
use sp_runtime::{
  offchain::{
    http,
    storage::StorageValueRef,
    storage_lock::{BlockAndTime, StorageLock},
    Duration,
  },
  traits::BlockNumberProvider,
};
use sp_std::{prelude::*, str};

impl<T: Config> Pallet<T> {
  /// Append a new number to the tail of the list, removing an element from the head if reaching
  ///   the bounded length.
  pub fn append_or_replace_number(number: u64) {
    Numbers::<T>::mutate(|numbers| {
      if numbers.len() == NUM_VEC_LEN {
        let _ = numbers.pop_front();
      }
      numbers.push_back(number);
      log::info!("Number vector: {:?}", numbers);
    });
  }

  /// Check if we have fetched the data before. If yes, we can use the cached version
  ///   stored in off-chain worker storage `storage`. If not, we fetch the remote info and
  ///   write the info into the storage for future retrieval.
  pub fn fetch_remote_info() -> Result<(), Error<T>> {
    // Create a reference to Local Storage value.
    // Since the local storage is common for all offchain workers, it's a good practice
    // to prepend our entry with the pallet name.
    let s_info = StorageValueRef::persistent(b"offchain-demo::hn-info");

    // Local storage is persisted and shared between runs of the offchain workers,
    // offchain workers may run concurrently. We can use the `mutate` function to
    // write a storage entry in an atomic fashion.
    //
    // With a similar API as `StorageValue` with the variables `get`, `set`, `mutate`.
    // We will likely want to use `mutate` to access
    // the storage comprehensively.
    //
    if let Ok(Some(info)) = s_info.get::<HackerNewsInfo>() {
      // hn-info has already been fetched. Return early.
      log::info!("cached hn-info: {:?}", info);
      return Ok(());
    }

    // Since off-chain storage can be accessed by off-chain workers from multiple runs, it is important to lock
    //   it before doing heavy computations or write operations.
    //
    // There are four ways of defining a lock:
    //   1) `new` - lock with default time and block exipration
    //   2) `with_deadline` - lock with default block but custom time expiration
    //   3) `with_block_deadline` - lock with default time but custom block expiration
    //   4) `with_block_and_time_deadline` - lock with custom time and block expiration
    // Here we choose the most custom one for demonstration purpose.
    let mut lock = StorageLock::<BlockAndTime<Self>>::with_block_and_time_deadline(
      b"offchain-demo::lock",
      LOCK_BLOCK_EXPIRATION,
      Duration::from_millis(LOCK_TIMEOUT_EXPIRATION),
    );

    // We try to acquire the lock here. If failed, we know the `fetch_n_parse` part inside is being
    //   executed by previous run of ocw, so the function just returns.
    if let Ok(_guard) = lock.try_lock() {
      match Self::fetch_n_parse() {
        Ok(info) => {
          s_info.set(&info);
        },
        Err(err) => {
          return Err(err);
        },
      }
    }
    Ok(())
  }

  /// Fetch from remote and deserialize the JSON to a struct
  pub fn fetch_n_parse() -> Result<HackerNewsInfo, Error<T>> {
    let resp_bytes = Self::fetch_from_remote().map_err(|e| {
      log::error!("fetch_from_remote error: {:?}", e);
      <Error<T>>::HttpFetchingError
    })?;

    let resp_str = str::from_utf8(&resp_bytes).map_err(|_| <Error<T>>::DeserializeToStrError)?;
    // Print out our fetched JSON string
    log::info!("fetch_n_parse: {}", resp_str);

    // Deserializing JSON to struct, thanks to `serde` and `serde_derive`
    let info: HackerNewsInfo = serde_json::from_str(&resp_str).map_err(|_| <Error<T>>::DeserializeToObjError)?;
    Ok(info)
  }

  /// This function uses the `offchain::http` API to query the remote endpoint information,
  ///   and returns the JSON response as vector of bytes.
  pub fn fetch_from_remote() -> Result<Vec<u8>, Error<T>> {
    // Initiate an external HTTP GET request. This is using high-level wrappers from `sp_runtime`.
    let request = http::Request::get(HTTP_REMOTE_REQUEST);

    // Keeping the offchain worker execution time reasonable, so limiting the call to be within 3s.
    let timeout = sp_io::offchain::timestamp().add(Duration::from_millis(FETCH_TIMEOUT_PERIOD));

    let pending = request
      .deadline(timeout) // Setting the timeout time
      .send() // Sending the request out by the host
      .map_err(|e| {
        log::error!("{:?}", e);
        <Error<T>>::HttpFetchingError
      })?;

    // By default, the http request is async from the runtime perspective. So we are asking the
    //   runtime to wait here
    // The returning value here is a `Result` of `Result`, so we are unwrapping it twice by two `?`
    //   ref: https://docs.substrate.io/rustdocs/latest/sp_runtime/offchain/http/struct.PendingRequest.html#method.try_wait
    let response = pending
      .try_wait(timeout)
      .map_err(|e| {
        log::error!("{:?}", e);
        <Error<T>>::HttpFetchingError
      })?
      .map_err(|e| {
        log::error!("{:?}", e);
        <Error<T>>::HttpFetchingError
      })?;

    if response.code != 200 {
      log::error!("Unexpected http request status code: {}", response.code);
      return Err(<Error<T>>::HttpFetchingError);
    }

    // Next we fully read the response body and collect it to a vector of bytes.
    Ok(response.body().collect::<Vec<u8>>())
  }

  pub fn offchain_signed_tx(block_number: T::BlockNumber) -> Result<(), Error<T>> {
    // We retrieve a signer and check if it is valid.
    //   Since this pallet only has one key in the keystore. We use `any_account()1 to
    //   retrieve it. If there are multiple keys and we want to pinpoint it, `with_filter()` can be chained,
    let signer = Signer::<T, T::AuthorityId>::any_account();

    // Translating the current block number to number and submit it on-chain
    let number: u64 = block_number.try_into().unwrap_or(0);

    // `result` is in the type of `Option<(Account<T>, Result<(), ()>)>`. It is:
    //   - `None`: no account is available for sending transaction
    //   - `Some((account, Ok(())))`: transaction is successfully sent
    //   - `Some((account, Err(())))`: error occured when sending the transaction
    let result = signer.send_signed_transaction(|_acct|
  // This is the on-chain function
  Call::submit_number_signed { number });

    // Display error if the signed tx fails.
    if let Some((acc, res)) = result {
      if res.is_err() {
        log::error!("failure: offchain_signed_tx: tx sent: {:?}", acc.id);
        return Err(<Error<T>>::OffchainSignedTxError);
      }
      // Transaction is sent successfully
      return Ok(());
    }

    // The case of `None`: no account is available for sending
    log::error!("No local account available");
    Err(<Error<T>>::NoLocalAcctForSigning)
  }

  pub fn offchain_unsigned_tx(block_number: T::BlockNumber) -> Result<(), Error<T>> {
    let number: u64 = block_number.try_into().unwrap_or(0);
    let call = Call::submit_number_unsigned { number };

    // `submit_unsigned_transaction` returns a type of `Result<(), ()>`
    //   ref: https://substrate.dev/rustdocs/v2.0.0/frame_system/offchain/struct.SubmitTransaction.html#method.submit_unsigned_transaction
    SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into()).map_err(|_| {
      log::error!("Failed in offchain_unsigned_tx");
      <Error<T>>::OffchainUnsignedTxError
    })
  }

  pub fn offchain_unsigned_tx_signed_payload(block_number: T::BlockNumber) -> Result<(), Error<T>> {
    // Retrieve the signer to sign the payload
    let signer = Signer::<T, T::AuthorityId>::any_account();

    let number: u64 = block_number.try_into().unwrap_or(0);

    // `send_unsigned_transaction` is returning a type of `Option<(Account<T>, Result<(), ()>)>`.
    //   Similar to `send_signed_transaction`, they account for:
    //   - `None`: no account is available for sending transaction
    //   - `Some((account, Ok(())))`: transaction is successfully sent
    //   - `Some((account, Err(())))`: error occured when sending the transaction
    if let Some((_, res)) = signer.send_unsigned_transaction(
      |acct| Payload {
        number,
        public: acct.public.clone(),
      },
      |payload, signature| Call::submit_number_unsigned_with_signed_payload { payload, signature },
    ) {
      return res.map_err(|_| {
        log::error!("Failed in offchain_unsigned_tx_signed_payload");
        <Error<T>>::OffchainUnsignedTxSignedPayloadError
      });
    }

    // The case of `None`: no account is available for sending
    log::error!("No local account available");
    Err(<Error<T>>::NoLocalAcctForSigning)
  }
}

impl<T: Config> BlockNumberProvider for Pallet<T> {
  type BlockNumber = T::BlockNumber;

  fn current_block_number() -> Self::BlockNumber {
    <frame_system::Pallet<T>>::block_number()
  }
}
