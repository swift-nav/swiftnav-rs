@Library("ci-jenkins") import com.swiftnav.ci.*

def context = new Context(context: this)
context.setRepo("swiftnav-rs")

pipeline {
  agent { node { label('docker.m') } }
  environment {
    SCCACHE_SIZE="100G"
    SCCACHE_DIR="/opt/sccache"
    SCCACHE_BUCKET="sccache-linux"
    SCCACHE_REGION="us-west-2"
    USER="jenkins"
  }
  options {
    timeout(time: 1, unit: 'HOURS')
    timestamps()
    // Keep builds for 30 days.
    buildDiscarder(logRotator(daysToKeepStr: '30'))
  }
  stages {
    stage('Check') {
      agent { dockerfile { reuseNode true } }
      steps {
        gitPrep()
        script {
          sh("cargo check")
        }
      }
    }
    stage('Build checks') {
      parallel {
        stage('Test') {
          agent { dockerfile { reuseNode true } }
          steps {
            script {
              sh("cargo test")
            }
          }
        }
        stage('Format') {
          agent { dockerfile { reuseNode true } }
          steps {
            script {
              sh("cargo fmt -- --check")
            }
          }
        }
        stage('Lint') {
          agent { dockerfile { reuseNode true } }
          steps {
            script {
              sh("cargo clippy")
            }
          }
        }
      }
    }
  }
}

