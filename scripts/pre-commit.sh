#!/bin/bash
# MiniCRM Pre-commit Hook
# 在提交前执行代码质量检查

set -e

echo "🔍 执行提交前代码质量检查..."

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 检查函数
check_command() {
    if ! command -v "$1" &> /dev/null; then
        echo -e "${RED}错误: $1 命令未找到${NC}"
        echo "请安装 $1 或确保它在PATH中"
        exit 1
    fi
}

# 检查必要的工具
echo -e "${BLUE}检查必要工具...${NC}"
check_command "cargo"
check_command "rustc"

# 1. 代码格式化检查
echo -e "${BLUE}1. 检查代码格式...${NC}"
if ! cargo fmt --all -- --check; then
    echo -e "${RED}❌ 代码格式检查失败${NC}"
    echo -e "${YELLOW}提示: 运行 'cargo fmt --all' 来格式化代码${NC}"
    exit 1
fi
echo -e "${GREEN}✅ 代码格式检查通过${NC}"

# 2. Clippy代码检查
echo -e "${BLUE}2. 执行Clippy代码检查...${NC}"
if ! cargo clippy --all-targets --all-features -- -D warnings; then
    echo -e "${RED}❌ Clippy检查失败${NC}"
    echo -e "${YELLOW}提示: 修复上述警告和错误后重新提交${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Clippy检查通过${NC}"

# 3. 编译检查
echo -e "${BLUE}3. 执行编译检查...${NC}"
if ! cargo check --all-targets --all-features; then
    echo -e "${RED}❌ 编译检查失败${NC}"
    exit 1
fi
echo -e "${GREEN}✅ 编译检查通过${NC}"

# 4. 单元测试
echo -e "${BLUE}4. 执行单元测试...${NC}"
if ! cargo test --all-features; then
    echo -e "${RED}❌ 单元测试失败${NC}"
    exit 1
fi
echo -e "${GREEN}✅ 单元测试通过${NC}"

# 5. 安全审计（如果cargo-audit可用）
if command -v cargo-audit &> /dev/null; then
    echo -e "${BLUE}5. 执行安全审计...${NC}"
    if ! cargo audit; then
        echo -e "${YELLOW}⚠️  安全审计发现问题，请检查${NC}"
        # 不阻止提交，但给出警告
    else
        echo -e "${GREEN}✅ 安全审计通过${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  cargo-audit未安装，跳过安全审计${NC}"
    echo -e "${YELLOW}提示: 运行 'cargo install cargo-audit' 来安装${NC}"
fi

# 6. 文档检查
echo -e "${BLUE}6. 检查文档生成...${NC}"
if ! cargo doc --all-features --no-deps --quiet; then
    echo -e "${RED}❌ 文档生成失败${NC}"
    exit 1
fi
echo -e "${GREEN}✅ 文档生成成功${NC}"

echo -e "${GREEN}🎉 所有检查通过，可以提交！${NC}"