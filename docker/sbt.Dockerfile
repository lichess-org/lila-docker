FROM sbtscala/scala-sbt:eclipse-temurin-alpine-25_36_1.11.6_3.7.3

ARG USER_ID
ARG GROUP_ID

RUN if [ "$USER_ID" != "1001" ] && [ "$USER_ID" != "0" ]; then \
        adduser -D -u $USER_ID -G $(id -gn sbtuser) -h $(eval echo ~sbtuser) newuser; \
        chown -R $USER_ID:$(id -gn sbtuser) $(eval echo ~sbtuser); \
    fi

CMD ["sbt"]
