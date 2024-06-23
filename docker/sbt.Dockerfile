FROM sbtscala/scala-sbt:eclipse-temurin-alpine-21.0.2_13_1.10.0_3.4.2

ARG USER_ID
ARG GROUP_ID

RUN if [ "$USER_ID" != "1001" ] && [ "$USER_ID" != "0" ]; then \
        adduser -D -u $USER_ID -G $(id -gn sbtuser) -h $(eval echo ~sbtuser) newuser; \
        chown -R $USER_ID:$(id -gn sbtuser) $(eval echo ~sbtuser); \
    fi

CMD ["sbt"]
