use super::header::*;
use derive_new::new;


#[derive(Clone, Debug, PartialEq, new)]
pub struct World {
  pub header: Header,
  // pub properties: Properties,
  // pub status: Status
}
