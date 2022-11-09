# Tkassa

In some orienteering clubs, members pay a share of their event fees. Sometimes late fees are paid solely by the members, and sometimes DNS ("Did Not Start") as well. Aggregating this data based on the event invoices and the event result lists can be very time consuming. Tkassa automates the task of extracting this data.

Tkassa uses the Eventor REST API to:

- get a list of all competitions for a given time period
- get a list of results specific to the querying club
- if any members were at the event or had pre-registered for the event:
    - get a list of entry fees at the event
    - get a list of event classes, containing the fees to apply for each class
    - get a list of pre-entries (including fees)
    - then, for each club member result, check if the member was pre-registered and if so calculate the fee, or, if not, check the event class and use those fees instead.
- finally, present a list of each active club member and a sub-list of all billable events for the time period.

The output is in tab-separated format and can be piped to a csv file, which can then be imported to Excel or similar.

## Pre-requisites

You will need an API key for Eventor to run this tool. There is a specific key for each club, and you will need to use the one for your club. The API key can be obtained from Eventor support, but please check if someone in your club maybe already asked them for the key.

You will also need to know your club ID number. This is listed as "Organisation ID" on the club ID page ("About the club") on the main Eventor site.

## Installation

Tkassa is written in Rust. Follow [these instructions](https://www.rust-lang.org/tools/install) to install Rust. Then, check out the git repository, change to that folder, and build with

    cargo build --release

Then, the tkassa binary will be at `target/release/tkassa`. A typical invocation (excluding the API key, see above) may look like:

    target/release/tkassa -q -i 42705,42490,24475,37349,40629 -o 224 APIKEY 2022-01-01 2022-11-08 

Replace `APIKEY` with your API key. Here a few events are ignored, mainly O-ringen (because O-ringen takes a long time to load and the fees are currently not paid over Eventor). The organisation ID is 224.

## Caches

When you run tkassa, there may be several thousand different queries to Eventor. In case there is a problem or you want to run the tool again, tkassa stores the result of each query in an XML file and the query results can in general be used again without accessing Eventor. You can specify where to put these files with the `-c` option. Default is `caches` in the `tkassa` folder.

On a related note, do not run the tool repeatedly with different cache settings, and maybe don't run it on Sunday evenings when Eventor is otherwise very busy.

Running the tool will take a lot of time (up to an hour), but when all the queries are cached it should complete in a couple of seconds.

## Known issues

Tkassa currently only targets the swedish version of Eventor. Adding options to instead query the norwegian or australian versions should be fairly straight-forward.

Setting up competition fees in Eventor can be complicated and sometimes organisers get it wrong. Tkassa tries to do its best, but for some events the fees may be incorrect. This tool is provided without any guarantees. Also, a broken fee structure may also cause the tool to crash. Feedback is greatly appreciated!
