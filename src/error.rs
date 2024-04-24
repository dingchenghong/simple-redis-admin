use fred::prelude::RedisError;
use salvo::{Writer, async_trait, prelude::StatusCode, writer::Text};



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    NotFound(String),
    TeraError(String),
    ParseError(String),
    RedisError(String),
    InternalServerError(String),
}

#[async_trait]
impl Writer for Error {
    async fn write(
        self, 
        _req: &mut salvo::Request,
        depot: &mut salvo::Depot,
        res: &mut salvo::Response,) {
        let status = match self {
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::TeraError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::ParseError(_) => StatusCode::BAD_REQUEST,
            Error::RedisError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let error_text = match self.clone() {
            Error::NotFound(m) => m,
            Error::TeraError(m) => m,
            Error::ParseError(m) => m,
            Error::RedisError(m) => m,
            Error::InternalServerError(m) => m,
        };

        res.set_status_code(status);
        if let Error::TeraError(e) = self {
            // We cannot use Tera
            res.render(e);
        } else {
            let tera = depot.obtain::<tera::Tera>().unwrap();
            let mut context = tera::Context::new();

            context.insert("error", &error_text);
            res.render(Text::Html(
                tera.render("error.html", &context)
                    .unwrap_or_else(|_| "无法渲染错误页面，请联系网站管理员".into()),
            ));
        }

    }
}

impl From<tera::Error> for Error {
    fn from(err:tera::Error) -> Self {
        tracing::error!("{:?}", err);
        Self::TeraError("无法渲染网页模板，请联系网站管理员".into())
    }
}

impl From<RedisError> for Error {
    fn from(value: RedisError) -> Self {
        tracing::error!("redis error: {:?}", value);
        Self::RedisError("redis操作发生错误, 请联系网站管理员".into())
    }
}