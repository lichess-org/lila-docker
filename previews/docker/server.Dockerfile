FROM sbtscala/scala-sbt:eclipse-temurin-25_1.x AS build

COPY /lila /lila
WORKDIR /lila
RUN TZ=UTC git log -1 --date=iso-strict-local --pretty='format:app.version.commit = "%H"%napp.version.date = "%ad"%napp.version.message = """%s"""%n' | tee conf/version.conf
RUN ./lila.sh stage

FROM eclipse-temurin:21-jre

WORKDIR /lila

COPY --from=build /lila/target/universal/stage/ /lila/
COPY --from=build /lila/conf/                   /lila/conf/

RUN chmod +x bin/lila

CMD ["bin/lila", "-Dconfig.file=/lila/conf/application.conf", "-Dlogger.file=/lila/conf/logger.dev.xml"]
