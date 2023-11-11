# syntax=docker/dockerfile:1

# Build the application from source
FROM golang:1.21 AS build-stage

WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download

COPY ./ ./

RUN CGO_ENABLED=0 GOOS=linux go build -o /junction-api

# Deploy the application binary into a lean image
FROM gcr.io/distroless/base-debian11 AS build-release-stage

WORKDIR /

COPY --from=build-stage /junction-api /junction-api
COPY --from=build-stage /app/dist /dist
COPY --from=build-stage /app/files /files

EXPOSE 3000

USER nonroot:nonroot

ENTRYPOINT ["/junction-api"]