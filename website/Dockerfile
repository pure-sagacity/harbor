FROM oven/bun:1.3.14-debian as builder

# Set the working directory inside the container
WORKDIR /app

# Copy package.json and bun.lock to the working directory
COPY package.json bun.lock ./

# Install dependencies using Bun
RUN bun install --frozen-lockfile

# Copy the rest of the application code to the working directory
COPY . .

# Build the application
RUN bun run build

# Use Caddy as the final image
FROM caddy:2.11.4-alpine

# Copy the built application from the builder stage to the Caddy image
COPY --from=builder /app/dist /usr/share/caddy

# Expose port 80 for the application
EXPOSE 80

# Start Caddy server using ./Caddyfile
CMD ["caddy", "run", "--config", "/etc/caddy/Caddyfile", "--adapter", "caddyfile"]