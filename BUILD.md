# Creating a build

## Prerequisites

The Near Firehose Indexer is an extension of the indexer framework provided by the NEAR core project.  We have a [fork](https://github.com/streamingfast/nearcore) of their codebase which needs to be modified in order to make some minor alterations. The patch we apply to their code can be found in this repository at nearcore.firehose.patch.

Therefore, whenever NEAR announces a new release, we need to update our fork to the latest version.  This is done by running the following commands in our fork of the NEAR core project:

```bash
git fetch upstream # this will fetch the latest tags, including the latest release
git checkout <target release> # checkout their tag
git checkout /dm/<target release> # checkout a new branch for our changes
git apply /path/to/nearcore.firehose.patch # apply our patch
git add . # add all the changes
git commit -m "apply near-firehose-indexer patch"
git tag <target release>-firehose # tag our changes
git push --atomic origin /dm/<target release> <target release>-firehose # push our changes
```

## Building the Near Firehose Indexer

Using the tag from the previous step, we can now build the Near Firehose Indexer.  This is done by doing the following:

1. Update the `nearcore` dependency in `Cargo.toml` to the tag we created in the previous step.
2. Run `cargo build --release` to build the Near Firehose Indexer and confirm that it builds successfully. If it does not build successfully, you will need to fix the errors and try again. Typically, between major releases, there are dependencies which need to be updated in to match the versions in the NEAR Core project.
3. Once you confirm that everything builds successfully, you can do the following:

```bash
git add .
git commit -m "update near to <target release>-firehose"
git tag <target release>-fire
git push --atomic origin develop <target release>-fire
```

Pushing this commit will automatically trigger a build in our CI/CD pipeline which will build the Near Firehose Indexer and publish it to our docker repository. (Note: This build takes approximately 30 minutes at the time of this writing)

You can now use this tag you have created to build a new version of the bundle image in the [firehose-near](https://github.com/streamingfast/firehose-near) project.