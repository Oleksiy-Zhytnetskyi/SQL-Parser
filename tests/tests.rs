use pest::Parser;
use sql_parser::*;

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Ok;

    #[test]
    fn test_aggregate_fn() -> anyhow::Result<()> {
        let parsed = Grammar::parse(Rule::aggregate_fn, "COUNT(_abc)");
        assert!(parsed.is_ok());
        Ok(())
    }

    #[test]
    fn test_aggregate_fn_with_distinct() -> anyhow::Result<()> {
        let parsed = Grammar::parse(Rule::aggregate_fn, "MAX(DISTINCT abc)");
        assert!(parsed.is_ok());
        Ok(())
    }

    #[test]
    fn test_aggregate_fn_with_invalid_indent_should_fail() -> anyhow::Result<()> {
        let parsed = Grammar::parse(Rule::aggregate_fn, "MAX(3ident)");
        assert!(parsed.is_err());
        Ok(())
    }

    #[test]
    fn test_aggregate_fn_with_double_distinct_should_fail() -> anyhow::Result<()> {
        let parsed = Grammar::parse(Rule::aggregate_fn, "MAX(DISTINCT DISTINCT abc)");
        assert!(parsed.is_err());
        Ok(())
    }

    #[test]
    fn test_aggregate_fn_with_ident_inbetween_double_distinct_should_fail() -> anyhow::Result<()> {
        let parsed = Grammar::parse(Rule::aggregate_fn, "MAX(DISTINCT abc DISTINCT)");
        assert!(parsed.is_err());
        Ok(())
    }
}
