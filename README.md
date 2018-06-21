`efr32xg1` register crate
--------------------------

This crate contains the svd2rust product of chip description files (SVDs)
provided for the EFR32xG1P family of microprocessors.

### Applicability

Device designations of EFR32 files appear to be structured like this:

    EFR32[family]G[series-and-config][performance][featureset]F[flashzize][temperaturegrade][package][pincount]

* The family part is typically a letter ("B" for "Blue Gecko", "M" for "Mighty
  Gecko", "F" for "Flex Gecko" etc.
* The series-and-config part is a one to two digit number indicating the series
  in the first digit and the confg in the second if present.
* Performance grade is a letter.
* Feature set is a numeric code related to radio frequencies.
* Flash size is given in kB.
* Temperature grade and package type are letters, pin count is numeric.

(Source: EFR32MG1 and EFR32FG14 data sheets. Note that these are the order
codes also used in the SVD file names and descriptions; designations printed on
the chips are encoded differently.)

Unlike in the EFM32 series, the family part does not indicate regular
peripheral internal revisions any more. (In EFM32, a Leopart Gecko would have
slightly different clocks than a Tiny Gecko); they only indicate differences in
the radio stacks (where funnily, Flex Gecko is the least flexible as it can
only do proprietary networking).

Given that radio components are not shown in the SVDs and svd2rust does not
touch memory sizes, all components but the series-and-config can be ignored for
matters of generating svd2rust code. The EFM32MG1P133F256GM48 is used as the
representative chip in the source, but the author assumes that all EFR32 chips
with series-and-config equal to "1" can be used with this crate.

Beware that mixing register crates between series-and-config versions will not
work; for example, the GPIO bit in the CMU_HFPERCLKEN0 register has moved
between 1 and 12.

### Usage

This crate has nothing special about the way it is used compared to any other
[svd2rust] crate, so see [its documentation] or [the generic chip
documentation].

[svd2rust]: https://github.com/japaric/svd2rust
[its documentation]: https://docs.rs/svd2rust/0.13.1/svd2rust/#peripheral-api
[the generic chip documentation]: https://studio.segger.com/packages/EFR32BG1P/CMSIS/Documents/EFR32xG1-ReferenceManual.pdf

### License

All code and text checked into this repository was written by chrysn
<chrysn@fsfe.org> and is published under the "MIT or Apache-2.0" terms
indicated in the Cargo file.

The copyright status of the SVD files used (and therefore the generated crate)
is subject to [pending clarification]; the author works under the assumption
that the data extracted is not subject to copyright, but is aware that the
resulting product still does not qualify for Free Software by some standards.

[pending clarification]: https://www.silabs.com/community/mcu/32-bit/forum.topic.html/license_for_svd_file-HuoY
