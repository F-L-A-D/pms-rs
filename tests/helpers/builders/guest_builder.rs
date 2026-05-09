use serde_json::{
    json,
    Value,
};

pub struct GuestBuilder {
    last_name: String,
    first_name: String,
    email: Option<String>,
    phone: Option<String>,
}

impl GuestBuilder {

    pub fn new() -> Self {

        Self {
            last_name:
                "Yamada".into(),

            first_name:
                "Taro".into(),

            email:
                None,

            phone:
                None,
        }
    }

    pub fn with_name(
        mut self,
        last_name: &str,
        first_name: &str,
    ) -> Self {

        self.last_name =
            last_name.into();

        self.first_name =
            first_name.into();

        self
    }

    pub fn with_email(
        mut self,
        email: &str,
    ) -> Self {

        self.email =
            Some(email.into());

        self
    }

    pub fn with_phone(
        mut self,
        phone: &str,
    ) -> Self {

        self.phone =
            Some(phone.into());

        self
    }

    pub fn build(self) -> Value {

        json!({
            "last_name":
                self.last_name,

            "first_name":
                self.first_name,

            "email":
                self.email,

            "phone":
                self.phone,
        })
    }
}