#!/bin/sh

set -eu

STATE_FILE="$HOME/.arch_achievements/pet_state"
QML_FILE="${TMPDIR:-/tmp}/achievix-overlay.qml"
PID_FILE="$HOME/.arch_achievements/overlay.pid"

mkdir -p "$HOME/.arch_achievements"

if [ -f "$PID_FILE" ] && kill -0 "$(cat "$PID_FILE")" 2>/dev/null; then
    echo "Achievix overlay is already running." >&2
    exit 0
fi

echo "$$" > "$PID_FILE"
trap 'rm -f "$PID_FILE"' EXIT INT TERM

arch-achieve pet >/dev/null 2>&1 || true

cat > "$QML_FILE" <<EOF
import QtQuick 2.15
import QtQuick.Window 2.15

Window {
    id: root
    width: 260
    height: 180
    visible: true
    color: "transparent"
    flags: Qt.FramelessWindowHint | Qt.WindowStaysOnTopHint | Qt.Tool
    x: Screen.width - width - 34
    y: Screen.height - height - 84

    property string stateUrl: "file://$STATE_FILE"
    property string petName: "Achievix Pet"
    property string mood: "loading"
    property string message: "Watching the system"
    property string activity: "starting overlay"

    function refresh() {
        var req = new XMLHttpRequest()
        req.open("GET", stateUrl)
        req.onreadystatechange = function() {
            if (req.readyState === XMLHttpRequest.DONE && req.status === 200) {
                var parts = req.responseText.trim().split("|")
                if (parts.length >= 4) {
                    petName = parts[0]
                    mood = parts[1]
                    message = parts[2]
                    activity = parts[3]
                }
            }
        }
        req.send()
    }

    Timer {
        interval: 3000
        running: true
        repeat: true
        onTriggered: root.refresh()
    }

    Rectangle {
        anchors.fill: parent
        radius: 18
        color: "#20242c"
        opacity: 0.9
        border.color: "#5fd1c7"
        border.width: 1
    }

    Column {
        anchors.fill: parent
        anchors.margins: 18
        spacing: 8

        Text {
            text: root.petName
            color: "#f5f7fb"
            font.pixelSize: 24
            font.bold: true
            elide: Text.ElideRight
            width: parent.width
        }

        Text {
            text: root.mood + " · " + root.activity
            color: "#5fd1c7"
            font.pixelSize: 13
            elide: Text.ElideRight
            width: parent.width
        }

        Text {
            text: root.message
            color: "#d8dee9"
            font.pixelSize: 14
            wrapMode: Text.WordWrap
            width: parent.width
            maximumLineCount: 4
        }
    }

    Component.onCompleted: refresh()
}
EOF

if command -v qml6 >/dev/null 2>&1; then
    exec qml6 "$QML_FILE"
elif command -v qmlscene >/dev/null 2>&1; then
    exec qmlscene "$QML_FILE"
elif command -v qml >/dev/null 2>&1; then
    exec qml "$QML_FILE"
fi

echo "No Qt QML runner found. Install qt6-declarative or qt5-declarative." >&2
exit 1
