# LogAI Deployment Guide

This guide covers deploying LogAI in various environments for production use.

## Table of Contents

- [Deployment Options](#deployment-options)
- [Docker Deployment](#docker-deployment)
- [Kubernetes Deployment](#kubernetes-deployment)
- [CI/CD Integration](#cicd-integration)
- [Monitoring Integration](#monitoring-integration)
- [Production Best Practices](#production-best-practices)
- [Scaling Considerations](#scaling-considerations)

## Deployment Options

### 1. Standalone Binary

The simplest deployment option for single-server use.

```bash
# Build release binary
cargo build --release

# Copy to target server
scp target/release/logai user@server:/usr/local/bin/

# Set permissions
ssh user@server 'chmod +x /usr/local/bin/logai'
```

### 2. Docker Container

Containerized deployment for consistency across environments.

### 3. Kubernetes

Orchestrated deployment for large-scale log analysis.

### 4. Serverless

Lambda/Cloud Functions for event-driven analysis.

## Docker Deployment

### Basic Dockerfile

```dockerfile
# Multi-stage build for smaller image
FROM rust:1.70 as builder

WORKDIR /app
COPY . .

# Build release binary
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/logai /usr/local/bin/logai

# Create non-root user
RUN useradd -m -u 1000 logai && \
    mkdir -p /home/logai/.logai && \
    chown -R logai:logai /home/logai

USER logai
WORKDIR /home/logai

ENTRYPOINT ["logai"]
CMD ["--help"]
```

### Build and Run

```bash
# Build image
docker build -t logai:latest .

# Run with local logs
docker run -v /var/log:/logs logai:latest investigate /logs/app.log --ai ollama

# Run with Ollama
docker run --network host \
  -v /var/log:/logs \
  logai:latest investigate /logs/app.log --ai ollama

# Run with OpenAI
docker run -e OPENAI_API_KEY="sk-..." \
  -v /var/log:/logs \
  logai:latest investigate /logs/app.log --ai openai
```

### Docker Compose

```yaml
version: "3.8"

services:
  ollama:
    image: ollama/ollama:latest
    ports:
      - "11434:11434"
    volumes:
      - ollama-data:/root/.ollama
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: 1
              capabilities: [gpu]

  logai:
    build: .
    depends_on:
      - ollama
    volumes:
      - /var/log:/logs:ro
      - ./config:/home/logai/.logai
    environment:
      - RUST_LOG=info
    command: investigate /logs/app.log --ai ollama --concurrency 5

volumes:
  ollama-data:
```

### Run with Docker Compose

```bash
# Start services
docker-compose up -d

# View logs
docker-compose logs -f logai

# Run one-time analysis
docker-compose run --rm logai investigate /logs/app.log --ai ollama

# Stop services
docker-compose down
```

## Kubernetes Deployment

### Deployment Manifest

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: logai
  namespace: monitoring
spec:
  replicas: 3
  selector:
    matchLabels:
      app: logai
  template:
    metadata:
      labels:
        app: logai
    spec:
      containers:
        - name: logai
          image: logai:latest
          imagePullPolicy: Always
          resources:
            requests:
              memory: "256Mi"
              cpu: "250m"
            limits:
              memory: "1Gi"
              cpu: "1000m"
          env:
            - name: RUST_LOG
              value: "info"
            - name: OPENAI_API_KEY
              valueFrom:
                secretKeyRef:
                  name: logai-secrets
                  key: openai-api-key
          volumeMounts:
            - name: logs
              mountPath: /logs
              readOnly: true
            - name: config
              mountPath: /home/logai/.logai
      volumes:
        - name: logs
          hostPath:
            path: /var/log
            type: Directory
        - name: config
          configMap:
            name: logai-config
```

### ConfigMap

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: logai-config
  namespace: monitoring
data:
  config.toml: |
    default_provider = "openai"

    [analysis]
    max_concurrency = 5
    enable_retry = true
    max_retries = 3
    enable_cache = true

    [providers.openai]
    enabled = true
    model = "gpt-4"
```

### Secret

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: logai-secrets
  namespace: monitoring
type: Opaque
stringData:
  openai-api-key: "sk-your-key-here"
  anthropic-api-key: "sk-ant-your-key-here"
```

### CronJob for Scheduled Analysis

```yaml
apiVersion: batch/v1
kind: CronJob
metadata:
  name: logai-hourly
  namespace: monitoring
spec:
  schedule: "0 * * * *" # Every hour
  jobTemplate:
    spec:
      template:
        spec:
          containers:
            - name: logai
              image: logai:latest
              args:
                - investigate
                - /logs/app.log
                - --ai
                - openai
                - --format
                - json
                - --limit
                - "10"
              env:
                - name: OPENAI_API_KEY
                  valueFrom:
                    secretKeyRef:
                      name: logai-secrets
                      key: openai-api-key
              volumeMounts:
                - name: logs
                  mountPath: /logs
                  readOnly: true
                - name: output
                  mountPath: /output
          volumes:
            - name: logs
              hostPath:
                path: /var/log
            - name: output
              persistentVolumeClaim:
                claimName: logai-output
          restartPolicy: OnFailure
```

### Deploy to Kubernetes

```bash
# Create namespace
kubectl create namespace monitoring

# Apply configurations
kubectl apply -f k8s/configmap.yaml
kubectl apply -f k8s/secret.yaml
kubectl apply -f k8s/deployment.yaml
kubectl apply -f k8s/cronjob.yaml

# Check status
kubectl get pods -n monitoring
kubectl logs -f deployment/logai -n monitoring

# Run manual analysis
kubectl run logai-manual --rm -it --restart=Never \
  --image=logai:latest \
  --namespace=monitoring \
  -- investigate /logs/app.log --ai openai
```

## CI/CD Integration

### GitHub Actions

```yaml
name: Log Analysis

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]
  schedule:
    - cron: "0 */6 * * *" # Every 6 hours

jobs:
  analyze-logs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Install LogAI
        run: cargo install --path .

      - name: Download logs
        run: |
          # Download logs from your infrastructure
          aws s3 cp s3://my-logs/app.log ./logs/

      - name: Analyze logs
        env:
          OPENAI_API_KEY: ${{ secrets.OPENAI_API_KEY }}
        run: |
          logai investigate logs/app.log \
            --ai openai \
            --format json \
            --limit 10 > analysis.json

      - name: Check for critical errors
        run: |
          if jq -e '.groups[] | select(.severity == "Critical")' analysis.json; then
            echo "Critical errors found!"
            exit 1
          fi

      - name: Upload analysis
        uses: actions/upload-artifact@v3
        with:
          name: log-analysis
          path: analysis.json

      - name: Create issue for critical errors
        if: failure()
        uses: actions/github-script@v6
        with:
          script: |
            github.rest.issues.create({
              owner: context.repo.owner,
              repo: context.repo.repo,
              title: 'Critical errors found in logs',
              body: 'Automated log analysis detected critical errors. See artifacts for details.',
              labels: ['bug', 'critical']
            })
```

### GitLab CI

```yaml
stages:
  - analyze
  - report

analyze-logs:
  stage: analyze
  image: rust:1.70
  script:
    - cargo install --path .
    - logai investigate logs/app.log --ai openai --format json > analysis.json
  artifacts:
    paths:
      - analysis.json
    expire_in: 1 week
  only:
    - schedules

report-critical:
  stage: report
  image: alpine:latest
  script:
    - apk add --no-cache jq
    - |
      if jq -e '.groups[] | select(.severity == "Critical")' analysis.json; then
        echo "Critical errors detected!"
        exit 1
      fi
  dependencies:
    - analyze-logs
  only:
    - schedules
```

### Jenkins Pipeline

```groovy
pipeline {
    agent any

    triggers {
        cron('H */6 * * *')  // Every 6 hours
    }

    environment {
        OPENAI_API_KEY = credentials('openai-api-key')
    }

    stages {
        stage('Setup') {
            steps {
                sh 'cargo install --path .'
            }
        }

        stage('Analyze Logs') {
            steps {
                sh '''
                    logai investigate /var/log/app.log \
                        --ai openai \
                        --format json \
                        --limit 10 > analysis.json
                '''
            }
        }

        stage('Check Results') {
            steps {
                script {
                    def analysis = readJSON file: 'analysis.json'
                    def critical = analysis.groups.findAll { it.severity == 'Critical' }

                    if (critical.size() > 0) {
                        error("Found ${critical.size()} critical errors!")
                    }
                }
            }
        }

        stage('Archive') {
            steps {
                archiveArtifacts artifacts: 'analysis.json', fingerprint: true
            }
        }
    }

    post {
        failure {
            emailext(
                subject: "Log Analysis Failed: ${env.JOB_NAME}",
                body: "Critical errors detected in logs. Check ${env.BUILD_URL}",
                to: "team@company.com"
            )
        }
    }
}
```

## Monitoring Integration

### Prometheus Metrics

Create a metrics exporter:

```rust
// src/metrics.rs
use prometheus::{Counter, Histogram, Registry};

pub struct Metrics {
    pub logs_parsed: Counter,
    pub errors_found: Counter,
    pub analysis_duration: Histogram,
}

impl Metrics {
    pub fn new(registry: &Registry) -> Self {
        let logs_parsed = Counter::new("logai_logs_parsed_total", "Total logs parsed").unwrap();
        let errors_found = Counter::new("logai_errors_found_total", "Total errors found").unwrap();
        let analysis_duration = Histogram::new("logai_analysis_duration_seconds", "Analysis duration").unwrap();

        registry.register(Box::new(logs_parsed.clone())).unwrap();
        registry.register(Box::new(errors_found.clone())).unwrap();
        registry.register(Box::new(analysis_duration.clone())).unwrap();

        Self {
            logs_parsed,
            errors_found,
            analysis_duration,
        }
    }
}
```

### Grafana Dashboard

```json
{
  "dashboard": {
    "title": "LogAI Monitoring",
    "panels": [
      {
        "title": "Logs Parsed",
        "targets": [
          {
            "expr": "rate(logai_logs_parsed_total[5m])"
          }
        ]
      },
      {
        "title": "Errors Found",
        "targets": [
          {
            "expr": "rate(logai_errors_found_total[5m])"
          }
        ]
      },
      {
        "title": "Analysis Duration",
        "targets": [
          {
            "expr": "histogram_quantile(0.95, rate(logai_analysis_duration_seconds_bucket[5m]))"
          }
        ]
      }
    ]
  }
}
```

### Alerting

```yaml
# Prometheus alert rules
groups:
  - name: logai
    rules:
      - alert: HighErrorRate
        expr: rate(logai_errors_found_total[5m]) > 10
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High error rate detected"
          description: "Error rate is {{ $value }} errors/sec"

      - alert: SlowAnalysis
        expr:
          histogram_quantile(0.95,
          rate(logai_analysis_duration_seconds_bucket[5m])) > 60
        for: 10m
        labels:
          severity: warning
        annotations:
          summary: "Slow log analysis"
          description: "95th percentile analysis time is {{ $value }}s"
```

## Production Best Practices

### 1. Configuration Management

```bash
# Use environment-specific configs
/etc/logai/
├── config.toml          # Base config
├── production.toml      # Production overrides
├── staging.toml         # Staging overrides
└── development.toml     # Development overrides

# Load with environment variable
export LOGAI_ENV=production
logai investigate logs.txt --ai openai
```

### 2. Secret Management

```bash
# Use secret management tools
# AWS Secrets Manager
export OPENAI_API_KEY=$(aws secretsmanager get-secret-value \
  --secret-id logai/openai-key \
  --query SecretString \
  --output text)

# HashiCorp Vault
export OPENAI_API_KEY=$(vault kv get -field=api_key secret/logai/openai)

# Kubernetes secrets
kubectl create secret generic logai-secrets \
  --from-literal=openai-api-key="sk-..."
```

### 3. Log Rotation

```bash
# Logrotate configuration
cat > /etc/logrotate.d/logai << 'EOF'
/var/log/logai/*.log {
    daily
    rotate 7
    compress
    delaycompress
    missingok
    notifempty
    create 0640 logai logai
    postrotate
        systemctl reload logai
    endscript
}
EOF
```

### 4. Resource Limits

```bash
# Systemd service with limits
cat > /etc/systemd/system/logai.service << 'EOF'
[Unit]
Description=LogAI Log Analysis Service
After=network.target

[Service]
Type=simple
User=logai
Group=logai
ExecStart=/usr/local/bin/logai investigate /var/log/app.log --ai ollama
Restart=always
RestartSec=10

# Resource limits
MemoryLimit=1G
CPUQuota=100%
TasksMax=100

# Security
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/log/logai

[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload
systemctl enable logai
systemctl start logai
```

### 5. Monitoring and Alerting

```bash
# Health check endpoint
curl http://localhost:8080/health

# Metrics endpoint
curl http://localhost:8080/metrics

# Set up alerts for:
# - High error rates
# - Slow analysis times
# - API failures
# - Resource exhaustion
```

## Scaling Considerations

### Horizontal Scaling

```yaml
# Kubernetes HPA
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: logai-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: logai
  minReplicas: 2
  maxReplicas: 10
  metrics:
    - type: Resource
      resource:
        name: cpu
        target:
          type: Utilization
          averageUtilization: 70
    - type: Resource
      resource:
        name: memory
        target:
          type: Utilization
          averageUtilization: 80
```

### Load Balancing

```nginx
# Nginx load balancer
upstream logai {
    least_conn;
    server logai-1:8080;
    server logai-2:8080;
    server logai-3:8080;
}

server {
    listen 80;

    location / {
        proxy_pass http://logai;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

### Distributed Processing

```bash
# Process different log sources on different instances
# Instance 1: Web server logs
logai investigate /var/log/nginx/*.log --ai ollama

# Instance 2: Application logs
logai investigate /var/log/app/*.log --ai ollama

# Instance 3: Database logs
logai investigate /var/log/postgresql/*.log --ai ollama
```

## Troubleshooting Deployment

### Check Service Status

```bash
# Docker
docker ps | grep logai
docker logs logai

# Kubernetes
kubectl get pods -n monitoring
kubectl logs -f deployment/logai -n monitoring
kubectl describe pod logai-xxx -n monitoring

# Systemd
systemctl status logai
journalctl -u logai -f
```

### Common Issues

1. **Out of Memory**: Increase memory limits or reduce concurrency
2. **API Rate Limits**: Lower concurrency or add delays
3. **Network Issues**: Check firewall rules and DNS
4. **Permission Errors**: Verify file permissions and user access

## Backup and Recovery

```bash
# Backup configuration
tar -czf logai-config-backup.tar.gz ~/.logai/

# Backup cache
tar -czf logai-cache-backup.tar.gz ~/.logai/cache/

# Restore
tar -xzf logai-config-backup.tar.gz -C ~/
tar -xzf logai-cache-backup.tar.gz -C ~/
```

## Security Hardening

```bash
# Run as non-root user
useradd -r -s /bin/false logai

# Restrict file permissions
chmod 600 ~/.logai/config.toml
chmod 700 ~/.logai/cache/

# Use read-only mounts
docker run -v /var/log:/logs:ro logai:latest

# Network isolation
docker network create --internal logai-net
```

## Further Reading

- [Architecture Documentation](ARCHITECTURE.md)
- [Troubleshooting Guide](TROUBLESHOOTING.md)
- [Security Policy](../SECURITY.md)
- [Contributing Guide](../CONTRIBUTING.md)
