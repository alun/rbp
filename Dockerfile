FROM frolvlad/alpine-glibc

ARG APP

# TODO understand why below still doesnt work 
# FROM alpine:latest
# RUN apk add --update libgcc gcompat libc6-compat

COPY release/${APP} /usr/local/bin/${APP}

ENV APP $APP
CMD ["ash", "-c", "exec ${APP}"]