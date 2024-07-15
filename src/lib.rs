use std::borrow::Cow;

const NEGATIVE_SI_SCALE: &[Scale<'static>] = &[
    Scale::new(1.0e-03, Cow::Borrowed("m")),
    Scale::new(1.0e-06, Cow::Borrowed("μ")),
    Scale::new(1.0e-09, Cow::Borrowed("n")),
    Scale::new(1.0e-12, Cow::Borrowed("p")),
    Scale::new(1.0e-15, Cow::Borrowed("f")),
    Scale::new(1.0e-18, Cow::Borrowed("a")),
    Scale::new(1.0e-21, Cow::Borrowed("z")),
    Scale::new(1.0e-24, Cow::Borrowed("y")),
    Scale::new(1.0e-27, Cow::Borrowed("r")),
    Scale::new(1.0e-30, Cow::Borrowed("q")),
];
const POSITIVE_SI_SCALE: &[Scale<'static>] = &[
    Scale::new(1.0e+03, Cow::Borrowed("k")),
    Scale::new(1.0e+06, Cow::Borrowed("M")),
    Scale::new(1.0e+09, Cow::Borrowed("G")),
    Scale::new(1.0e+12, Cow::Borrowed("T")),
    Scale::new(1.0e+15, Cow::Borrowed("P")),
    Scale::new(1.0e+18, Cow::Borrowed("E")),
    Scale::new(1.0e+21, Cow::Borrowed("Z")),
    Scale::new(1.0e+24, Cow::Borrowed("Y")),
    Scale::new(1.0e+27, Cow::Borrowed("R")),
    Scale::new(1.0e+30, Cow::Borrowed("Q")),
];
pub const SI_SCALE: Scales<'static> = Scales::new(NEGATIVE_SI_SCALE, POSITIVE_SI_SCALE);

const POSITIVE_BINARY_SCALE: &[Scale<'static>] = &[
    Scale::new(1024.0, Cow::Borrowed("ki")),
    Scale::new(1048576.0, Cow::Borrowed("Mi")),
    Scale::new(1073741824.0, Cow::Borrowed("Gi")),
    Scale::new(1099511627776.0, Cow::Borrowed("Ti")),
    Scale::new(1125899906842624.0, Cow::Borrowed("Pi")),
    Scale::new(1152921504606846976.0, Cow::Borrowed("Ei")),
    Scale::new(1180591620717411303424.0, Cow::Borrowed("Zi")),
    Scale::new(1208925819614629174706176.0, Cow::Borrowed("Yi")),
    Scale::new(1237940039285380274899124224.0, Cow::Borrowed("Ri")),
    Scale::new(1267650600228229401496703205376.0, Cow::Borrowed("Qi")),
];
pub const BINARY_SCALE: Scales<'static> = Scales::new(&[], POSITIVE_BINARY_SCALE);

#[derive(Debug, Clone)]
pub struct Scale<'a> {
    factor: f64,
    prefix: Cow<'a, str>,
}

impl<'a> Scale<'a> {
    #[inline]
    pub const fn new(factor: f64, prefix: Cow<'a, str>) -> Self {
        Self { factor, prefix }
    }
}

#[derive(Clone, Debug)]
pub struct Scales<'a> {
    negatives: &'a [Scale<'a>],
    positives: &'a [Scale<'a>],
}

impl<'a> Scales<'a> {
    pub const fn new(negatives: &'a [Scale<'a>], positives: &'a [Scale<'a>]) -> Self {
        Self {
            negatives,
            positives,
        }
    }

    fn get_negative_scale(&'a self, absolute: f64) -> Option<&'a Scale<'a>> {
        for current in self.negatives.iter() {
            if absolute >= current.factor {
                return Some(current);
            }
        }
        self.negatives.last()
    }

    fn get_positive_scale(&'a self, absolute: f64) -> Option<&'a Scale<'a>> {
        let mut previous = None;
        for current in self.positives.iter() {
            if absolute < current.factor {
                return previous;
            }
            previous = Some(current);
        }
        previous
    }

    pub fn get_scale(&'a self, value: f64) -> Option<&'a Scale<'a>> {
        let absolute = f64::abs(value);
        if absolute < 1.0 {
            self.get_negative_scale(absolute)
        } else {
            self.get_positive_scale(absolute)
        }
    }

    pub fn into_scaled(&'a self, options: &'a Options<'a>, value: f64) -> ScaledValue<'a> {
        if let Some(scale) = self.get_scale(value) {
            ScaledValue {
                value: value / scale.factor,
                scale: Some(scale),
                options,
            }
        } else {
            ScaledValue {
                value,
                scale: None,
                options,
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ScaledValue<'a> {
    value: f64,
    scale: Option<&'a Scale<'a>>,
    options: &'a Options<'a>,
}

impl<'a> std::fmt::Display for ScaledValue<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.options.force_sign {
            write!(f, "{:+.width$}", self.value, width = self.options.decimals)?;
        } else {
            write!(f, "{:.width$}", self.value, width = self.options.decimals)?;
        }
        if self.scale.is_some() || self.options.unit.is_some() {
            f.write_str(self.options.separator.as_ref())?;
        }
        if let Some(scale) = self.scale {
            f.write_str(scale.prefix.as_ref())?;
        }
        if let Some(ref unit) = self.options.unit {
            f.write_str(unit.as_ref())?;
        }
        Ok(())
    }
}

/// Set of options used for formating numbers.
#[derive(Clone, Debug)]
pub struct Options<'a> {
    decimals: usize,
    separator: Cow<'a, str>,
    unit: Option<Cow<'a, str>>,
    force_sign: bool,
}

impl<'a> Default for Options<'a> {
    fn default() -> Self {
        Self {
            decimals: 2,
            separator: Cow::Borrowed(" "),
            unit: None,
            force_sign: false,
        }
    }
}

impl<'a> Options<'a> {
    pub const fn new(decimals: usize, separator: Cow<'a, str>, unit: Option<Cow<'a, str>>) -> Self {
        Self {
            decimals,
            separator,
            unit,
            force_sign: false,
        }
    }

    /// Forces the sign to be displayed.
    #[inline]
    pub fn set_force_sign(&mut self, force_sign: bool) {
        self.force_sign = force_sign;
    }

    /// Forces the sign to be displayed.
    pub const fn with_force_sign(mut self, force_sign: bool) -> Self {
        self.force_sign = force_sign;
        self
    }

    /// Sets the number of decimals to display.
    #[inline]
    pub fn set_decimals(&mut self, decimals: usize) {
        self.decimals = decimals;
    }

    /// Sets the number of decimals to display.
    pub const fn with_decimals(mut self, decimals: usize) -> Self {
        self.decimals = decimals;
        self
    }

    /// Sets the expected unit, like `B` for bytes or `g` for grams.
    #[inline]
    pub fn set_unit<U: Into<Cow<'a, str>>>(&mut self, unit: U) {
        self.unit = Some(unit.into());
    }

    /// Sets the expected unit, like `B` for bytes or `g` for grams.
    pub fn with_unit<U: Into<Cow<'a, str>>>(mut self, unit: U) -> Self {
        self.set_unit(unit);
        self
    }

    /// Sets the separator between the number and the preffix.
    #[inline]
    pub fn set_separator<U: Into<Cow<'a, str>>>(&mut self, separator: U) {
        self.separator = separator.into();
    }

    /// Sets the separator between the number and the preffix.
    pub fn with_separator<U: Into<Cow<'a, str>>>(mut self, separator: U) -> Self {
        self.set_separator(separator);
        self
    }
}

/// Structure containing options and scales used to format numbers
/// with the right scale preffix, separators and units.
#[derive(Clone, Debug)]
pub struct Formatter<'a> {
    scales: Scales<'a>,
    options: Options<'a>,
}

impl<'a> Formatter<'a> {
    /// Create a formatter with a given scale set and some options
    #[inline]
    pub fn new(scales: Scales<'a>, options: Options<'a>) -> Self {
        Self { scales, options }
    }

    /// Forces the sign to be displayed.
    #[inline]
    pub fn set_force_sign(&mut self, force_sign: bool) {
        self.options.set_force_sign(force_sign);
    }

    /// Forces the sign to be displayed.
    pub fn with_force_sign(mut self, force_sign: bool) -> Self {
        self.options.set_force_sign(force_sign);
        self
    }

    /// Sets the number of decimals to display.
    #[inline]
    pub fn set_decimals(&mut self, decimals: usize) {
        self.options.decimals = decimals;
    }

    /// Sets the number of decimals to display.
    pub fn with_decimals(mut self, decimals: usize) -> Self {
        self.options.set_decimals(decimals);
        self
    }

    /// Sets the expected unit, like `B` for bytes or `g` for grams.
    #[inline]
    pub fn set_unit<U: Into<Cow<'a, str>>>(&mut self, unit: U) {
        self.options.unit = Some(unit.into());
    }

    /// Sets the expected unit, like `B` for bytes or `g` for grams.
    pub fn with_unit<U: Into<Cow<'a, str>>>(mut self, unit: U) -> Self {
        self.options.set_unit(unit);
        self
    }

    /// Sets the separator between the number and the preffix.
    #[inline]
    pub fn set_separator<U: Into<Cow<'a, str>>>(&mut self, separator: U) {
        self.options.separator = separator.into();
    }

    /// Sets the separator between the number and the preffix.
    pub fn with_separator<U: Into<Cow<'a, str>>>(mut self, separator: U) -> Self {
        self.options.set_separator(separator);
        self
    }

    /// Formats a number and returns a scaled value that can be displayed.
    #[inline]
    pub fn format(&'a self, value: f64) -> ScaledValue<'a> {
        self.scales.into_scaled(&self.options, value)
    }
}

impl Formatter<'static> {
    /// Formatter that uses the SI format style
    ///
    /// ```rust
    /// use human_number::Formatter;
    ///
    /// let formatter = Formatter::si();
    /// let result = format!("{}", formatter.format(4_234.0));
    /// assert_eq!(result, "4.23 k");
    /// let result = format!("{}", formatter.format(0.012_34));
    /// assert_eq!(result, "12.34 m");
    ///
    /// let formatter = Formatter::si().with_unit("g").with_separator("").with_decimals(1);
    /// let result = format!("{}", formatter.format(4_234.0));
    /// assert_eq!(result, "4.2kg");
    /// let result = format!("{}", formatter.format(0.012_34));
    /// assert_eq!(result, "12.3mg");
    /// ```
    pub fn si() -> Self {
        Formatter {
            scales: SI_SCALE,
            options: Options::<'static>::default(),
        }
    }

    /// Formatter that uses the binary format style
    ///
    /// ```rust
    /// use human_number::Formatter;
    ///
    /// let formatter = Formatter::binary().with_unit("B");
    /// let result = format!("{}", formatter.format(4_320_133.0));
    /// assert_eq!(result, "4.12 MiB");
    /// ```
    pub fn binary() -> Self {
        Formatter {
            scales: BINARY_SCALE,
            options: Options::<'static>::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getting_scale() {
        assert!(SI_SCALE.get_scale(1.0).is_none());
        assert_eq!(SI_SCALE.get_scale(1000.0).unwrap().prefix, "k");
        assert_eq!(SI_SCALE.get_scale(0.10).unwrap().prefix, "m");
    }

    #[test_case::test_case(0.005, "5.000 m"; "should small number")]
    #[test_case::test_case(100.0, "100.000"; "should number")]
    #[test_case::test_case(5_432_100.0, "5.432 M"; "should format big number")]
    fn format_si_values_with_decimals(value: f64, expected: &'static str) {
        let formatter = Formatter::si().with_decimals(3);
        let result = format!("{}", formatter.format(value));
        assert_eq!(result, expected);
    }

    #[test_case::test_case(0.005, "5.00🦀m"; "should small number")]
    #[test_case::test_case(100.0, "100.00"; "should number")]
    #[test_case::test_case(5_432_100.0, "5.43🦀M"; "should format big number")]
    fn format_si_values_with_separator(value: f64, expected: &'static str) {
        let formatter = Formatter::si().with_separator("🦀");
        let result = format!("{}", formatter.format(value));
        assert_eq!(result, expected);
    }

    #[test_case::test_case(0.005, "5.00 m"; "should small number")]
    #[test_case::test_case(100.0, "100.00"; "should number")]
    #[test_case::test_case(5_432_100.0, "5.43 M"; "should format big number")]
    fn format_si_values_without_unit(value: f64, expected: &'static str) {
        let formatter = Formatter::si();
        let result = format!("{}", formatter.format(value));
        assert_eq!(result, expected);
    }

    #[test_case::test_case(0.005, "5.00 mg"; "should small number")]
    #[test_case::test_case(100.0, "100.00 g"; "should number")]
    #[test_case::test_case(5_432_100.0, "5.43 Mg"; "should format big number")]
    fn format_si_values_with_unit(value: f64, expected: &'static str) {
        let formatter = Formatter::si().with_unit("g");
        let result = format!("{}", formatter.format(value));
        assert_eq!(result, expected);
    }

    #[test_case::test_case(100.0, "100.00"; "should number")]
    #[test_case::test_case(4096.0, "4.00 ki"; "should format kilo number")]
    #[test_case::test_case(4194304.0, "4.00 Mi"; "should format mega number")]
    fn format_binary_values_without_unit(value: f64, expected: &'static str) {
        let formatter = Formatter::binary();
        let result = format!("{}", formatter.format(value));
        assert_eq!(result, expected);
    }

    #[test_case::test_case(100.0, "100.00 B"; "should number")]
    #[test_case::test_case(4096.0, "4.00 kiB"; "should format kilo number")]
    #[test_case::test_case(4194304.0, "4.00 MiB"; "should format mega number")]
    fn format_binary_values_with_unit(value: f64, expected: &'static str) {
        let formatter = Formatter::binary().with_unit("B");
        let result = format!("{}", formatter.format(value));
        assert_eq!(result, expected);
    }

    #[test]
    fn format_with_sign() {
        let scales: Scales = Scales::new(&[], &[]);
        let options = Options::default().with_force_sign(true);
        let formatter = Formatter::new(scales, options);
        assert_eq!(format!("{}", formatter.format(-1.0)), "-1.00");
        assert_eq!(format!("{}", formatter.format(1.0)), "+1.00");
    }

    #[test]
    fn format_with_non_static() {
        let negatives = vec![Scale::new(0.000_001, String::from("x").into())];
        let positives = vec![Scale::new(1_000.0, String::from("k").into())];
        let scales: Scales = Scales::new(&negatives, &positives);
        let options = Options::default()
            .with_unit("🦀")
            .with_separator("")
            .with_decimals(1);
        let formatter = Formatter::new(scales, options);
        assert_eq!(format!("{}", formatter.format(1_234.567)), "1.2k🦀");
        assert_eq!(
            format!("{}", formatter.format(0.000_012_345_678)),
            "12.3x🦀"
        );
    }
}
