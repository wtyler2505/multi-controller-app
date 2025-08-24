# Multi-stage Dockerfile for Multi-Controller App

# Stage 1: Node.js Development Environment
FROM node:20-alpine AS node-dev
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
EXPOSE 3000
CMD ["npm", "run", "dev"]

# Stage 2: .NET Build Environment
FROM mcr.microsoft.com/dotnet/sdk:8.0-windowsservercore AS dotnet-build
WORKDIR /app
COPY app/*.csproj ./
RUN dotnet restore
COPY app/ ./
RUN dotnet publish -c Release -r win-x64 --self-contained true -p:PublishAot=true -o /publish

# Stage 3: Production Runtime
FROM mcr.microsoft.com/windows/nanoserver:ltsc2022 AS production
WORKDIR /app
COPY --from=dotnet-build /publish ./
EXPOSE 8080
ENTRYPOINT ["MultiControllerApp.exe"]