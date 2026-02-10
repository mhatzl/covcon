use crate::{cfg::ConversionConfig, cobertura};

pub fn convert(
    cfg: &ConversionConfig,
) -> Result<String, Box<dyn core::error::Error + Send + Sync + 'static>> {
    match (&cfg.in_fmt, &cfg.in_data_fmt) {
        (crate::format::CoverageFormat::CoberturaV4, crate::cfg::DataFormat::Xml) => {
            let coverage = cobertura::schema::Coverage::try_from(cfg.in_content.as_str())?;

            match (&cfg.out_fmt, &cfg.out_data_fmt) {
                (crate::format::CoverageFormat::CoberturaV4, crate::cfg::DataFormat::Xml) => {
                    // nothing to convert
                    Ok(cfg.in_content.clone())
                }
                (crate::format::CoverageFormat::CoberturaV4, crate::cfg::DataFormat::Json) => {
                    let json_cov = crate::cobertura::no_xml::Coverage::from(coverage);
                    Ok(json_cov.try_into_json()?)
                }
                _ => {
                    panic!("Selected conversion is currently unsupported.")
                }
            }
        }
        (crate::format::CoverageFormat::CoberturaV4, crate::cfg::DataFormat::Json) => {
            let _coverage = cobertura::no_xml::Coverage::try_from_json(&cfg.in_content)?;

            match (&cfg.out_fmt, &cfg.out_data_fmt) {
                (crate::format::CoverageFormat::CoberturaV4, crate::cfg::DataFormat::Json) => {
                    // nothing to convert
                    Ok(cfg.in_content.clone())
                }
                _ => {
                    panic!("Selected conversion is currently unsupported.")
                }
            }
        }
        _ => {
            panic!("Selected conversion is currently unsupported.")
        }
    }
}

pub fn convert_to_json(
    cfg: &ConversionConfig,
) -> Result<serde_json::Value, Box<dyn core::error::Error + Send + Sync + 'static>> {
    match (&cfg.in_fmt, &cfg.in_data_fmt) {
        (crate::format::CoverageFormat::CoberturaV4, crate::cfg::DataFormat::Xml) => {
            let coverage = cobertura::schema::Coverage::try_from(cfg.in_content.as_str())?;

            match (&cfg.out_fmt, &cfg.out_data_fmt) {
                (crate::format::CoverageFormat::CoberturaV4, crate::cfg::DataFormat::Json) => {
                    let json_cov = crate::cobertura::no_xml::Coverage::from(coverage);
                    Ok(json_cov.try_into_json_value()?)
                }
                _ => {
                    panic!("Selected conversion is currently unsupported.")
                }
            }
        }
        (crate::format::CoverageFormat::CoberturaV4, crate::cfg::DataFormat::Json) => {
            let coverage = cobertura::no_xml::Coverage::try_from_json(&cfg.in_content)?;

            match (&cfg.out_fmt, &cfg.out_data_fmt) {
                (crate::format::CoverageFormat::CoberturaV4, crate::cfg::DataFormat::Json) => {
                    Ok(coverage.try_into_json_value()?)
                }
                _ => {
                    panic!("Selected conversion is currently unsupported.")
                }
            }
        }
        _ => {
            panic!("Selected conversion is currently unsupported.")
        }
    }
}
