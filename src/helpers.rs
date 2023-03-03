use crate::misc::{consts, structs};

use google_sheets4 as sheets4;
use sheets4::{hyper, hyper_rustls, oauth2, Error, Sheets, api};

pub fn sort_absences_details(absences_details: &mut Vec<structs::AbsenceInfo>) {
    absences_details.sort_by_key(|absence_details| {
        (
            absence_details.rank as u32,
            absence_details.name.to_lowercase(),
        )
    });
}

pub async fn get_spreadsheet(spreadsheet_id: &str) -> Result<(hyper::Response<hyper::Body>,api::Spreadsheet),Error> {
    let secret = oauth2::read_application_secret("tokens/priv-key.json")
        .await
        .expect(consts::PRIVATE_KEY_NOT_FOUND_MESSAGE);

    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .persist_tokens_to_disk("tokens/tokencache.json")
    .build()
    .await
    .expect(consts::AUTHENTICATOR_FAILED_ERROR_MESSAGE);

    let hub = Sheets::new(
        hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()),
        auth,
    );

    hub
    .spreadsheets()
    .get(spreadsheet_id)
    .doit()
    .await
}
