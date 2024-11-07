use pest::Parser;
use sql_parser::*;

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Ok;

    #[test]
    fn test_ident_column() -> anyhow::Result<()> {
        let parsed = Grammar::parse(Rule::column, "my_v4l1d_id3nt");
        assert!(parsed.is_ok());
        Ok(())
    }

    #[test]
    fn test_aggregate_fn_column() -> anyhow::Result<()> {
        let parsed = Grammar::parse(Rule::column, "COUNT(_abc)");
        assert!(parsed.is_ok());
        Ok(())
    }

    #[test]
    fn test_aggregate_fn_with_distinct_column() -> anyhow::Result<()> {
        let parsed = Grammar::parse(Rule::column, "MAX(DISTINCT abc)");
        assert!(parsed.is_ok());
        Ok(())
    }

    #[test]
    fn test_invalid_ident_column_should_fail() -> anyhow::Result<()> {
        let parsed = Grammar::parse(Rule::column, "3ident");
        assert!(parsed.is_err());
        Ok(())
    }

    #[test]
    fn test_aggregate_fn_with_invalid_indent_column_should_fail() -> anyhow::Result<()> {
        // Does not fail!
        let parsed = Grammar::parse(Rule::column, "MAX(3ident)");
        assert!(parsed.is_err());
        Ok(())
    }
}
