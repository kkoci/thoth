use actix_web::{http::StatusCode, HttpRequest, Responder};
use paperclip::actix::web::HttpResponse;
use paperclip::actix::OperationModifier;
use paperclip::util::{ready, Ready};
use paperclip::v2::models::{DefaultOperationRaw, Either, Response};
use paperclip::v2::schema::Apiv2Schema;
use std::str::FromStr;
use thoth_api::errors::{ThothError, ThothResult};
use thoth_client::Work;

use crate::xml::{Onix3Oapen, Onix3ProjectMuse, XmlSpecification};

pub(crate) trait AsRecord {}
impl AsRecord for Work {}
impl AsRecord for Vec<Work> {}

pub struct CsvThoth {}

pub(crate) enum MetadataSpecification {
    Onix3ProjectMuse(Onix3ProjectMuse),
    Onix3Oapen(Onix3Oapen),
    CsvThoth(CsvThoth),
}

pub(crate) struct MetadataRecord<T: AsRecord> {
    data: T,
    specification: MetadataSpecification,
}

impl<T> MetadataRecord<T>
where
    T: AsRecord,
{
    pub(crate) fn new(specification: MetadataSpecification, data: T) -> Self {
        MetadataRecord {
            data,
            specification,
        }
    }

    fn content_type(&self) -> &'static str {
        match &self.specification {
            MetadataSpecification::Onix3ProjectMuse(_) => "text/xml; charset=utf-8",
            MetadataSpecification::Onix3Oapen(_) => "text/xml; charset=utf-8",
            MetadataSpecification::CsvThoth(_) => "text/csv; charset=utf-8",
        }
    }
}

impl MetadataRecord<Work> {
    fn generate(self) -> ThothResult<String> {
        match self.specification {
            MetadataSpecification::Onix3ProjectMuse(onix3_project_muse) => {
                onix3_project_muse.generate(self.data)
            }
            MetadataSpecification::Onix3Oapen(_) => unimplemented!(),
            MetadataSpecification::CsvThoth(_) => unimplemented!(),
        }
    }
}

impl MetadataRecord<Vec<Work>> {
    fn generate(self) -> ThothResult<String> {
        unimplemented!()
    }
}

macro_rules! paperclip_responder {
    ($record_type:ty) => {
        impl Responder for MetadataRecord<$record_type>
        where
            actix_web::dev::Body: From<String>,
        {
            type Error = ThothError;
            type Future = Ready<ThothResult<HttpResponse>>;

            fn respond_to(self, _: &HttpRequest) -> Self::Future {
                ready(Ok(HttpResponse::build(StatusCode::OK)
                    .content_type(self.content_type())
                    .header("Content-Disposition", "attachment")
                    .body(self.generate().unwrap())))
            }
        }
    };
}

paperclip_responder!(Work);
paperclip_responder!(Vec<Work>);

impl<T: AsRecord> Apiv2Schema for MetadataRecord<T> {}

impl<T> OperationModifier for MetadataRecord<T>
where
    T: AsRecord,
{
    fn update_response(op: &mut DefaultOperationRaw) {
        let status: StatusCode = StatusCode::OK;
        op.responses.insert(
            status.as_str().into(),
            Either::Right(Response {
                description: status.canonical_reason().map(ToString::to_string),
                schema: None,
                ..Default::default()
            }),
        );
    }
}

impl FromStr for MetadataSpecification {
    type Err = ThothError;

    fn from_str(input: &str) -> ThothResult<Self> {
        match input {
            "onix_3.0::project_muse" => {
                Ok(MetadataSpecification::Onix3ProjectMuse(Onix3ProjectMuse {}))
            }
            "onix_3.0::oapen" => Ok(MetadataSpecification::Onix3Oapen(Onix3Oapen {})),
            "csv::thoth" => Ok(MetadataSpecification::CsvThoth(CsvThoth {})),
            _ => Err(ThothError::InvalidMetadataSpecification(input.to_string())),
        }
    }
}
