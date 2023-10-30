## Developer Release Guide

When a new version of nearcore is released, the near-firehose-indexer should be updated to use the latest version of nearcore. This is done by updating the `Cargo.toml` file to point to the new version of nearcore.

Once this is done, compile the project locally using `make release` and check that the project compiles locally.  This is important, because the Github action takes about 30 minutes to compile, and so compiling locally allows you to potentially save a lot of time.

After you have validated that the project compiles locally, tag your commit with the following format: `{{nearcore-tag}}-fire`.

The Github action will then compile the project and push the docker image to Dockerhub.  This image can then be used in the `firehose-near` project's Github action to build the bundle image.

### Example:

Let's say Near releases a new version of nearcore with tag `1.99.0-rc.3`
1. Update the `Cargo.toml` file package version to `1.99.0-rc.3`
2. In `Cargo.toml`, update all `nearcore` dependencies' `rev` tag to `1.99.0-rc.3`
3. Run `make release` locally to validate that the project compiles
4. Commit changes and tag commit with `1.99.0-rc.3-fire`
5. Push commit.

### Notes

* When nearcore releases a new set of release candidates (ie: an `-rc.1` release), this usually involves many more changes than a normal release. Normally, you will need to also update the `rust-toolchain.toml` file to match their version.  
* If compilation still fails, then you will need to dig deeper into the reasons why. Start with the compilation errors and work backwards from there.  
* One thing you might try is to git checkout the original nearcore project at the given tag, and try to compile their project. It is not inconceivable that nearcore has a bug in their code that prevents compilation.
* If nearcore does not compile, you can try to reach out to the Near team on their Discord in the rust-support channel.  They are usually very helpful.
* We use a multi-stage Dockerfile which compiles the `near-firehose-indexer` app in Docker using the Ubuntu 20.04 image.  One day, this might need to be updated to a more recent version of Ubuntu.  If so, you will need to update the Dockerfile accordingly.  The nearcore project still uses 18.04, so we should be good for a while.

### Then what?

If everything has gone as planned, the Github action will have built a Docker image and notified the cloudbuild-notif Slack channel.  You can now use this image in order to build the bundle image in the `firehose-near` project, using the `Build Docker Image` Github action.

### Where to deploy?

In the release notes on nearcore, you will see a note which indicates where to deploy in the `CODE_COLOR` section.  This section is of the form `CODE_{color}_{TESTNET/MAINNET}`, where the color represents the urgency of the release.  

For example, `CODE_RED_MAINNET` means that the release is urgent and should be deployed to mainnet as soon as possible.  `CODE_YELLOW_TESTNET` means that the release is not urgent, but should be deployed to testnet as soon as possible.  `CODE_GREEN_TESTNET` means that the release is not urgent, and can be deployed to testnet at your leisure.

If the `PROTOCOL_UPGRADE` section is set to `TRUE`, then this is a required upgrade and should be done as soon as possible because otherwise the node will be unable to get blocks from the network.

If the `DATABASE_UPGRADE` section is set to `TRUE`, this means when the node is started, it will perform a database upgrade.  This means that when the node starts, it might take some time for blocks to start flowing.  This is normal and expected behavior.  If you are not sure if the migration still happening or not, you can perform `dstat` on the node and see if there are still disk operations happening.  If there are, then the migration is likely still happening.