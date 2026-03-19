# Requirements
###### This application should fulfill the following:
- A file with the name `trigerror_{interface}_{time}.pcap` should be generated. The timestamp is the time of the first error.
- The location of the file is the `cwd` if not specified differently thru the CLI or config file.
- The app should listen on the specified interfaces/ports and start recording when an error in the specified
  protocol happened.
- The configuration parameters should be:
  - interface(s)
  - which protocol(s) should trigger the error
  - max amount of packets before the error
  - max amount of time before the error
  - max amount of packets after the error
  - max amount of time after the error
  - max size of file
  - path where the `.pcap`/`.pcapng` file(s) should be written
  - behavior in case an additional error happens while still capturing (retrigger)
  - max amount of retriggers
- App should not quit when it's done recording one file but keep producing files as long as it's running.
- The app will only be stopped explicitly by the user.
- The capturing part and the protocol validating part should be maximally decoupled (only interfacing via a `trait`).
- {nice to have} During the lifetime of the app give out the amount of packets and the amount of files generated and the number of errors captured.
- The app should be able to listen on multiple interfaces at the same time each in one separate thread.
  - If an error happens on one interface, that should trigger a recording on all running trigerrors.
  - The type of error (local, external) should be specified in the file name.
  - {nice to have} print that information into the console.
