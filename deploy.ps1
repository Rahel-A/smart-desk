$ErrorActionPreference = "Stop"
$TARGET_HOST="desk1@smartdesk1"
$TARGET_PATH="/home/desk1/smartdesk"
$TARGET_ARCH="aarch64-unknown-linux-gnu"
# $SOURCE_PATH="./target/${TARGET_ARCH}/debug/smartdesk"
$SOURCE_PATH="./target/${TARGET_ARCH}/release/smartdesk"

cargo zigbuild --release --target=${TARGET_ARCH}
# cargo zigbuild --target=${TARGET_ARCH}
ssh -t ${TARGET_HOST} sudo systemctl stop smart-desk.service
scp ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}
if($?)
{
ssh -t ${TARGET_HOST} sudo systemctl restart smart-desk.service
}