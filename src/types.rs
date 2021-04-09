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

#[cfg(test)]
mod tests {
    use crate::types::ValidityResponse;

    #[test]
    fn test_json_parsing() {
        let routinator_validity_http_endpoint_output_sample = r#"
{
  "validated_route": {
    "route": {
      "origin_asn": "AS43996",
      "prefix": "5.57.16.0/24"
    },
    "validity": {
      "state": "valid",
      "description": "At least one VRP Matches the Route Prefix",
      "VRPs": {
        "matched": [
          {
            "asn": "AS43996",
            "prefix": "5.57.16.0/24",
            "max_length": "24"
          }

        ],
        "unmatched_as": [
          {
            "asn": "AS26415",
            "prefix": "5.57.16.0/24",
            "max_length": "24"
          }
,
          {
            "asn": "AS19905",
            "prefix": "5.57.16.0/24",
            "max_length": "24"
          }

        ],
        "unmatched_length": [
          {
            "asn": "AS43996",
            "prefix": "5.57.16.0/21",
            "max_length": "21"
          }
,
          {
            "asn": "AS43996",
            "prefix": "5.57.16.0/22",
            "max_length": "22"
          }

        ]      }
    }
  }
}
"#;

        let json: ValidityResponse = serde_json::from_str(routinator_validity_http_endpoint_output_sample).unwrap();

        assert_eq!(&json.validated_route.route.origin_asn, "AS43996");
        assert_eq!(&json.validated_route.route.prefix, "5.57.16.0/24");

        assert_eq!(&json.validated_route.validity.state, "valid");
        assert_eq!(
            &json.validated_route.validity.description,
            "At least one VRP Matches the Route Prefix"
        );

        assert_eq!(json.validated_route.validity.vrps.matched.len(), 1);
        assert_eq!(&json.validated_route.validity.vrps.matched[0].asn, "AS43996");
        assert_eq!(&json.validated_route.validity.vrps.matched[0].prefix, "5.57.16.0/24");
        assert_eq!(&json.validated_route.validity.vrps.matched[0].max_length, "24");

        assert_eq!(json.validated_route.validity.vrps.unmatched_as.len(), 2);
        assert_eq!(&json.validated_route.validity.vrps.unmatched_as[0].asn, "AS26415");
        assert_eq!(
            &json.validated_route.validity.vrps.unmatched_as[0].prefix,
            "5.57.16.0/24"
        );
        assert_eq!(&json.validated_route.validity.vrps.unmatched_as[0].max_length, "24");
        assert_eq!(&json.validated_route.validity.vrps.unmatched_as[1].asn, "AS19905");
        assert_eq!(
            &json.validated_route.validity.vrps.unmatched_as[1].prefix,
            "5.57.16.0/24"
        );
        assert_eq!(&json.validated_route.validity.vrps.unmatched_as[1].max_length, "24");

        assert_eq!(json.validated_route.validity.vrps.unmatched_length.len(), 2);
        assert_eq!(&json.validated_route.validity.vrps.unmatched_length[0].asn, "AS43996");
        assert_eq!(
            &json.validated_route.validity.vrps.unmatched_length[0].prefix,
            "5.57.16.0/21"
        );
        assert_eq!(&json.validated_route.validity.vrps.unmatched_length[0].max_length, "21");
        assert_eq!(&json.validated_route.validity.vrps.unmatched_length[1].asn, "AS43996");
        assert_eq!(
            &json.validated_route.validity.vrps.unmatched_length[1].prefix,
            "5.57.16.0/22"
        );
        assert_eq!(&json.validated_route.validity.vrps.unmatched_length[1].max_length, "22");
    }
}