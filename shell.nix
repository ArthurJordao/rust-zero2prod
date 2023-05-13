with (import <nixpkgs> {});

stdenv.mkDerivation {
  name = "postgres-env";
  buildInputs = [darwin.apple_sdk.frameworks.SystemConfiguration pkgconfig openssl libiconv];

  nativeBuildInputs = [
    (postgresql_15.withPackages (p: [ p.postgis ]))
  ];

  postgresConf =
    writeText "postgresql.conf"
      ''
        # Add Custom Settings
        log_min_messages = warning
        log_min_error_statement = error
        log_min_duration_statement = 100  # ms
        log_connections = on
        log_disconnections = on
        log_duration = on
        #log_line_prefix = '[] '
        log_timezone = 'UTC'
        log_statement = 'all'
        log_directory = 'pg_log'
        log_filename = 'postgresql-%Y-%m-%d_%H%M%S.log'
        logging_collector = on
        log_min_error_statement = error
        max_connections = 1000
      '';


  # ENV Variables
  LD_LIBRARY_PATH = "${geos}/lib:${gdal}/lib";
  PGDATA = "${toString ./.}/.pg";
  PGPORT = 5555;
  PGUSER = "postgres";
  PGDBNAME = "newsletter";

  # Post Shell Hook
  shellHook = ''
    echo "Using ${postgresql_15.name}."

    # Setup: other env variables
    export PGHOST="$PGDATA"
    # Setup: DB
    [ ! -d $PGDATA ] && pg_ctl initdb -o "-U $PGUSER" && cat "$postgresConf" >> $PGDATA/postgresql.conf

    export DATABASE_URL="postgres://$PGUSER@localhost:$PGPORT/$PGDBNAME"
  '';
}
