FROM ghcr.io/lichess-org/lila-docker/picfit:v1

COPY assets/coach.png /uploads/coach.png
COPY assets/streamer.png /uploads/streamer.png

ENTRYPOINT ["/picfit", "-c", "/mnt/config.json"]
