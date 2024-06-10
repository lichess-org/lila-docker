FROM sbtscala/scala-sbt:eclipse-temurin-alpine-21.0.2_13_1.10.0_3.4.2

# add a new user with the same UID as the host user but permissions same as sbtuser in the container
ARG USER_ID
ARG GROUP_ID

RUN adduser -D -u $USER_ID -G $(id -gn sbtuser) -h $(eval echo ~sbtuser) newuser && \
    chown -R newuser:$(id -gn sbtuser) $(eval echo ~sbtuser)

CMD sbt
