#!/bin/bash

# 默认值设置
ARCH="riscv64"
LOG="warn"
QEMU_LOG="n"
TYPE="all"

# 解析参数
for arg in "$@"; do
    case $arg in
        ARCH=*)
            ARCH="${arg#*=}"
            ;;
        LOG=*)
            LOG="${arg#*=}"
            ;;
        QEMU_LOG=*)
            QEMU_LOG="${arg#*=}"
            ;;
        TYPE=*)
            TTYPE="${arg#*=}"
            ;;
    esac
done

echo "Architecture: $ARCH"
echo "Log level: $LOG"
echo "QEMU log: $QEMU_LOG"
echo "Test type: $TTYPE"

get_sudo() {
    # 检查是否已经有 sudo 权限
    if sudo -n true 2>/dev/null; then
        echo "已有 sudo 权限"
        return 0
    fi

    # 最多尝试 3 次
    local max_attempts=3
    local attempt=1

    while [ $attempt -le $max_attempts ]; do
        echo "请求 sudo 权限 (尝试 $attempt/$max_attempts)"
        if sudo -v; then
            echo "sudo 权限获取成功"
            # 保持 sudo 权限活跃
            sudo -v
            return 0
        fi
        echo "权限获取失败，请重试"
        ((attempt++))
    done

    echo "获取 sudo 权限失败，已达到最大尝试次数"
    return 1
}

check_installation() {
    if which riscv64-linux-musl-gcc > /dev/null 2>&1; then
        echo "已安装 riscv64-linux-musl-gcc"
        return 0
    else
        echo "未安装 riscv64-linux-musl-gcc"
        return 1
    fi
}

install_musl_riscv64() {
	if check_installation; then
		return 0
	fi
	
    # 克隆仓库
    cd ~
	git clone https://github.com/richfelker/musl-cross-make.git || {
        echo "git clone 失败"
        return 1
    }

    # 进入目录并配置
    cd musl-cross-make || return 1
    cp config.mak.list config.mak || return 1
    
    # 添加配置信息
    # printf "TARGET = riscv64-linux-musl\nOUTPUT = /opt/musl_riscv64\n" >> config.mak
    sed -i '15i\riscv64-linux-musl' config.mak
    sed -i '22i\OUTPUT = /opt/musl_riscv64' config.mak

    # 编译和安装
    make || {
        echo "make 失败"
        return 1
    }
    
    sudo make install || {
        echo "make install 失败"
        return 1
    }

    # 添加环境变量
    if ! grep -q "/opt/musl_riscv64/bin" ~/.zshrc; then
        echo 'export PATH=$PATH:/opt/musl_riscv64/bin' >> ~/.zshrc
        source ~/.zshrc
    fi

    echo "musl riscv64 工具链安装完成"
	rm -rf musl-cross-make
    return 0
}

check_branch() {
    local branch_name=$1
    if [ "$(git rev-parse --abbrev-ref HEAD)" = "$branch_name" ]; then
        echo "当前在 $branch_name 分支"
        return 0
    else
        echo "不在 $branch_name 分支"
		git switch $branch_name || {
			echo "切换分支失败"
			return 1
		}
		echo "切换到 $branch_name 分支"
        return 0
    fi
}

# 调用函数
get_sudo
install_musl_riscv64
check_branch "mocklibc"

dynamic_test() {
    cd ./payload
    echo "开始测试静态编译"
    python build.py static
    cd ..
    echo "开始运行"
    make defconfig ARCH=riscv64
    make A=examples/loader ARCH=$ARCH LOG=$LOG QEMU_LOG=$QEMU_LOG run
}

static_test() {
    cd ./payload
    echo "开始测试动态编译"
    python build.py dynamic
    cd ..
    echo "开始运行"
    make defconfig ARCH=riscv64
    make A=examples/loader ARCH=$ARCH LOG=$LOG QEMU_LOG=$QEMU_LOG run
}

echo "开始测试"

if [ "$TTYPE" = "dynamic" ]; then
    dynamic_test
elif [ "$TTYPE" = "static" ]; then
    static_test
elif [ "$TTYPE" = "all" ]; then
    dynamic_test
    static_test
else
    echo "测试失败"
fi