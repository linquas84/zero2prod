use crate::domain::{SubscriberName, subscriber_email::SubscriberEmail};

pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
