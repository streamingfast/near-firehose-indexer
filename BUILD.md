# Creating a build

## Building the Near Firehose Indexer

To build the Near Firehose Indexer, 

1. Update the `nearcore` dependency in `Cargo.toml` to the tag representing the version you want to build.
2. Run `cargo build` locally to build the project and confirm that it compiles successfully. If it does not build successfully, you will need to fix the errors and try again. Typically, between major releases, there are dependencies which need to be updated in order to match the versions in the NEAR Core project.
3. Once you confirm that everything builds successfully, you can do the following:

```bash
git add .
git commit -m "update near to <target release>-firehose"
git tag <target release>-fire
git push --atomic origin develop <target release>-fire
```

Pushing this commit will automatically trigger a build in our pipeline which will build the Near Firehose Indexer and publish it to our docker repository. (Note: This build takes approximately 30 minutes at the time of this writing)

You can now use the tag you have created in this step to build a new version of the bundle image in the [firehose-near](https://github.com/streamingfast/firehose-near) project.