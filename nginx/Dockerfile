# /app/build is shared from frontend build step in docker-compose
FROM nginx:stable-alpine
COPY nginx.default.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
