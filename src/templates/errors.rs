use askama::Template;

/// Generic error page template
#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate {
    pub title: String,
    pub message: String,
    pub details: Option<String>,
    pub suggestions: Vec<String>,
    pub error_code: Option<String>,
    pub back_url: String,
    pub home_url: String,
}

impl ErrorTemplate {
    /// Create error template for different scenarios
    pub fn for_not_found() -> Self {
        Self {
            title: "صفحه پیدا نشد".to_string(),
            message: "متأسفانه صفحه یا اطلاعات مورد نظر شما یافت نشد.".to_string(),
            details: None,
            suggestions: vec![
                "از منوی بالا برای دسترسی به بخش‌های مختلف استفاده کنید".to_string(),
                "آدرس صفحه را بررسی و دوباره امتحان کنید".to_string(),
                "اگر از طریق لینکی وارد شده‌اید، ممکن است لینک قدیمی باشد".to_string(),
            ],
            error_code: Some("404".to_string()),
            back_url: "javascript:history.back()".to_string(),
            home_url: "/".to_string(),
        }
    }

    pub fn for_unauthorized() -> Self {
        Self {
            title: "عدم دسترسی".to_string(),
            message: "برای مشاهده این صفحه باید وارد سیستم شوید.".to_string(),
            details: Some("نشست کاربری شما منقضی شده یا هنوز وارد سیستم نشده‌اید.".to_string()),
            suggestions: vec![
                "از دکمه زیر برای ورود به سیستم استفاده کنید".to_string(),
                "اگر قبلاً وارد شده بودید، ممکن است نشست شما منقضی شده باشد".to_string(),
            ],
            error_code: Some("401".to_string()),
            back_url: "/login".to_string(),
            home_url: "/login".to_string(),
        }
    }

    pub fn for_forbidden() -> Self {
        Self {
            title: "دسترسی ممنوع".to_string(),
            message: "شما اجازه دسترسی به این بخش را ندارید.".to_string(),
            details: Some("این بخش فقط برای مدیران سیستم قابل دسترسی است.".to_string()),
            suggestions: vec![
                "اگر نیاز به دسترسی دارید، با مدیر سیستم تماس بگیرید".to_string(),
                "از منوی اصلی برای دسترسی به بخش‌های مجاز استفاده کنید".to_string(),
            ],
            error_code: Some("403".to_string()),
            back_url: "/".to_string(),
            home_url: "/".to_string(),
        }
    }

    pub fn for_database_error() -> Self {
        Self {
            title: "خطای پایگاه داده".to_string(),
            message: "مشکلی در ذخیره یا بازیابی اطلاعات پیش آمده است.".to_string(),
            details: Some("این مشکل موقتی است و معمولاً با تلاش مجدد حل می‌شود.".to_string()),
            suggestions: vec![
                "چند لحظه صبر کنید و دوباره امتحان کنید".to_string(),
                "مرورگر خود را رفرش کنید (F5)".to_string(),
                "اگر مشکل ادامه داشت، با پشتیبانی تماس بگیرید".to_string(),
            ],
            error_code: Some("500".to_string()),
            back_url: "javascript:location.reload()".to_string(),
            home_url: "/".to_string(),
        }
    }

    pub fn for_duplicate_entry(field: &str) -> Self {
        let message = match field {
            "username" => "این نام کاربری قبلاً ثبت شده است",
            "email" => "این آدرس ایمیل قبلاً ثبت شده است",
            _ => "این مورد قبلاً در سیستم ثبت شده است",
        };

        Self {
            title: "اطلاعات تکراری".to_string(),
            message: message.to_string(),
            details: Some(
                "هر مورد باید منحصر به فرد باشد و نمی‌توان آن را دوباره ثبت کرد.".to_string(),
            ),
            suggestions: vec![
                "اطلاعات وارد شده را بررسی و تغییر دهید".to_string(),
                "اگر قبلاً این اطلاعات را ثبت کرده‌اید، می‌توانید آن را ویرایش کنید".to_string(),
                "از بخش جستجو برای یافتن موارد موجود استفاده کنید".to_string(),
            ],
            error_code: None,
            back_url: "javascript:history.back()".to_string(),
            home_url: "/".to_string(),
        }
    }

    pub fn for_validation_error(details: String) -> Self {
        Self {
            title: "خطای اعتبارسنجی".to_string(),
            message: "اطلاعات وارد شده معتبر نیست.".to_string(),
            details: Some(details),
            suggestions: vec![
                "اطلاعات فرم را بررسی و اصلاح کنید".to_string(),
                "مطمئن شوید همه فیلدهای اجباری را پر کرده‌اید".to_string(),
                "از فرمت صحیح برای ایمیل و شماره تلفن استفاده کنید".to_string(),
            ],
            error_code: None,
            back_url: "javascript:history.back()".to_string(),
            home_url: "/".to_string(),
        }
    }
}
