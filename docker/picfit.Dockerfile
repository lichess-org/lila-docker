FROM carbrex/thoas-picfit:latest

COPY assets/coach.png /uploads/coach.png
COPY assets/streamer.png /uploads/streamer.png

ENTRYPOINT ["/picfit", "-c", "/mnt/config.json"]
