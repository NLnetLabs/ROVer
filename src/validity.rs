use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Route {
    pub origin_asn: String,
    pub prefix: String,
}

#[derive(Deserialize, Debug)]
pub struct AddressOrigin {
    pub asn: String,
    pub prefix: String,
    pub max_length: String,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Deserialize, Debug)]
pub struct VRPs {
    pub matched: Vec<AddressOrigin>,
    pub unmatched_as: Vec<AddressOrigin>,
    pub unmatched_length: Vec<AddressOrigin>,
}

#[derive(Deserialize, Debug)]
pub struct Validity {
    pub state: String,
    pub reason: Option<String>,
    pub description: String,
    #[serde(rename = "VRPs")]
    pub vrps: VRPs,
}

#[derive(Deserialize, Debug)]
pub struct ValidatedRoute {
    pub route: Route,
    pub validity: Validity,
}

#[derive(Deserialize, Debug)]
pub struct ValidityResponse {
    pub validated_route: ValidatedRoute,
}
