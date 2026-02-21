#!/bin/bash
#
# 初始化脚本模板
# 此脚本用于设置开发环境并启动应用
#
# 使用方法：
# 1. 根据项目需求修改此脚本
# 2. 确保脚本有执行权限：chmod +x init.sh
# 3. 运行：./init.sh
#

set -e  # 遇到错误立即退出

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 日志函数
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 检查命令是否存在
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# ===========================================
# 配置区域 - 根据项目需求修改
# ===========================================

# 项目类型：nodejs, python, go, rust, other
PROJECT_TYPE="nodejs"

# 开发服务器端口
DEV_PORT=3000

# 是否需要安装依赖
INSTALL_DEPS=true

# 启动命令
START_CMD="npm run dev"

# 健康检查 URL（可选）
HEALTH_CHECK_URL="http://localhost:${DEV_PORT}"

# ===========================================
# 安装依赖
# ===========================================

install_dependencies() {
    if [ "$INSTALL_DEPS" = false ]; then
        log_info "跳过依赖安装"
        return
    fi

    log_info "安装项目依赖..."

    case "$PROJECT_TYPE" in
        nodejs)
            if [ -f "package.json" ]; then
                if command_exists npm; then
                    npm install
                elif command_exists yarn; then
                    yarn install
                elif command_exists pnpm; then
                    pnpm install
                else
                    log_error "未找到 npm/yarn/pnpm"
                    exit 1
                fi
            else
                log_warn "未找到 package.json，跳过依赖安装"
            fi
            ;;

        python)
            if [ -f "requirements.txt" ]; then
                pip install -r requirements.txt
            elif [ -f "pyproject.toml" ]; then
                pip install -e .
            else
                log_warn "未找到 Python 依赖文件，跳过依赖安装"
            fi
            ;;

        go)
            if [ -f "go.mod" ]; then
                go mod download
            else
                log_warn "未找到 go.mod，跳过依赖安装"
            fi
            ;;

        rust)
            if [ -f "Cargo.toml" ]; then
                cargo build
            else
                log_warn "未找到 Cargo.toml，跳过依赖安装"
            fi
            ;;

        *)
            log_info "未知项目类型，跳过依赖安装"
            ;;
    esac

    log_info "依赖安装完成"
}

# ===========================================
# 环境检查
# ===========================================

check_environment() {
    log_info "检查开发环境..."

    # 检查端口是否被占用
    if command_exists lsof; then
        if lsof -Pi :$DEV_PORT -sTCP:LISTEN -t >/dev/null 2>&1; then
            log_warn "端口 $DEV_PORT 已被占用，尝试终止..."
            lsof -ti:$DEV_PORT | xargs kill -9 2>/dev/null || true
            sleep 2
        fi
    fi

    log_info "环境检查完成"
}

# ===========================================
# 启动开发服务器
# ===========================================

start_server() {
    log_info "启动开发服务器..."

    case "$PROJECT_TYPE" in
        nodejs)
            # 检查是否有并发运行
            if command_exists npx; then
                # 如果有多个命令需要并发运行，使用 npm-run-all 或 concurrently
                if grep -q "\"dev:all\"" package.json 2>/dev/null; then
                    npm run dev:all
                else
                    eval "$START_CMD"
                fi
            else
                eval "$START_CMD"
            fi
            ;;

        python)
            if [ -f "main.py" ]; then
                python main.py
            elif [ -f "app.py" ]; then
                python app.py
            else
                log_error "未找到 Python 入口文件"
                exit 1
            fi
            ;;

        go)
            if [ -f "main.go" ]; then
                go run main.go
            else
                log_error "未找到 Go 入口文件"
                exit 1
            fi
            ;;

        rust)
            cargo run
            ;;

        *)
            log_info "使用自定义启动命令：$START_CMD"
            eval "$START_CMD"
            ;;
    esac
}

# ===========================================
# 健康检查
# ===========================================

health_check() {
    if [ -z "$HEALTH_CHECK_URL" ]; then
        return
    fi

    log_info "等待服务器启动..."

    # 等待最多 30 秒
    for i in {1..30}; do
        if command_exists curl; then
            if curl -s -f "$HEALTH_CHECK_URL" >/dev/null 2>&1; then
                log_info "服务器启动成功！"
                log_info "访问地址：$HEALTH_CHECK_URL"
                return
            fi
        fi

        # 简单的端口检查
        if command_exists nc && nc -z localhost $DEV_PORT 2>/dev/null; then
            log_info "服务器已在端口 $DEV_PORT 启动"
            return
        fi

        sleep 1
    done

    log_warn "健康检查超时，但服务器可能仍在启动"
}

# ===========================================
# 主流程
# ===========================================

main() {
    log_info "========================================="
    log_info "开发环境初始化"
    log_info "========================================="

    check_environment
    install_dependencies

    log_info "========================================="
    log_info "启动开发服务器..."
    log_info "========================================="

    # 在后台启动服务器
    start_server &
    SERVER_PID=$!

    # 等待服务器启动
    sleep 3
    health_check

    log_info "========================================="
    log_info "初始化完成！"
    log_info "========================================="
    log_info "服务器 PID: $SERVER_PID"
    log_info "按 Ctrl+C 停止服务器"

    # 保持脚本运行
    wait $SERVER_PID
}

# 执行主流程
main
