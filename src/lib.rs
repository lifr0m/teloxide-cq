mod serialize;
mod deserialize;

pub use deserialize::Deserialize;
use futures::future::BoxFuture;
use futures::FutureExt;
pub use serialize::Serialize;
use teloxide::types::CallbackQuery;

pub fn serialize_cq_data(args: Vec<&dyn Serialize>) -> String {
    let data = args.into_iter()
        .fold(Vec::new(), |mut buf, value| {
            value.serialize(&mut buf);
            buf
        });
    base128::encode(&data)
}

pub fn filter_cq(target: u64) -> impl Fn(CallbackQuery) -> BoxFuture<'static, Option<Vec<u8>>> {
    move |cq: CallbackQuery| async move {
        let data = cq.data?;
        let data = base128::decode(&data);
        let mut data = data.as_slice();

        if u64::deserialize(&mut data).ok()? == target {
            Some(data.to_vec())
        } else {
            None
        }
    }.boxed()
}
