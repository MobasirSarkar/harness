#!/usr/bin/env python3
import sys
import json
import os
import subprocess

# Read stdin payload from Antigravity CLI
try:
    input_text = sys.stdin.read()
    data = json.loads(input_text) if input_text.strip() else {}
except Exception:
    data = {}

# 1. Model display
model_obj = data.get("model", {})
if isinstance(model_obj, dict):
    model_name = model_obj.get("display_name") or model_obj.get("id") or "Antigravity"
else:
    model_name = str(model_obj) if model_obj else "Antigravity"

# Clean up model name
if "Gemini" in model_name:
    model_disp = model_name.replace("Gemini ", "GEMINI-").upper()
else:
    model_disp = model_name.upper()

# 2. Context Window usage percentage
cw = data.get("context_window", {})
if isinstance(cw, dict):
    pct = cw.get("used_percentage")
    if pct is None:
        used = cw.get("current_usage") or cw.get("used_tokens") or 0
        total = cw.get("total_token_limit") or cw.get("max_tokens") or 2000000
        pct = (used / total * 100) if total else 0
else:
    pct = 0

try:
    pct_int = int(round(float(pct)))
except (ValueError, TypeError):
    pct_int = 0

# Color coding for context usage
if pct_int < 50:
    ctx_color = "\033[38;5;82m"   # Soft Green
elif pct_int < 80:
    ctx_color = "\033[38;5;214m"  # Amber/Yellow
else:
    ctx_color = "\033[38;5;196m"  # Red

reset = "\033[0m"
sep = "\033[38;5;240m│\033[0m"

# 3. Directory display
cwd = data.get("cwd") or data.get("workspace") or os.getcwd()
home = os.path.expanduser("~")
if cwd == home:
    short_cwd = "~"
elif cwd.startswith(home + "/"):
    short_cwd = "~/" + cwd[len(home)+1:]
else:
    short_cwd = cwd

# Truncate path if too long
if len(short_cwd) > 35:
    parts = short_cwd.split("/")
    short_cwd = f"{parts[0]}/.../{parts[-1]}"

# 4. Git Branch & Status
git_str = ""
try:
    branch = subprocess.check_output(
        ["git", "rev-parse", "--abbrev-ref", "HEAD"],
        stderr=subprocess.DEVNULL,
        cwd=cwd if os.path.isdir(cwd) else None
    ).decode().strip()

    if branch:
        status = subprocess.check_output(
            ["git", "status", "--porcelain"],
            stderr=subprocess.DEVNULL,
            cwd=cwd if os.path.isdir(cwd) else None
        ).decode().strip()
        dirty = " \033[38;5;208m[DIRTY]\033[0m" if status else ""
        git_str = f" {sep} \033[38;5;147m GIT: {branch}\033[0m{dirty}"
except Exception:
    pass

# 5. Rust Harness CLI Indicator
harness_bin = os.path.expanduser("~/.local/bin/harness-cli")
if os.path.isfile(harness_bin) and os.access(harness_bin, os.X_OK):
    try:
        harness_out = subprocess.check_output(
            [harness_bin, "--status-short"],
            stderr=subprocess.DEVNULL
        ).decode().strip()
        harness_str = f" {sep} {harness_out}" if harness_out else f" {sep} \033[38;5;51m🦀 HARNESS: RUST v2.0\033[0m"
    except Exception:
        harness_str = f" {sep} \033[38;5;51m🦀 HARNESS: RUST v2.0\033[0m"
else:
    harness_str = ""

# 6. Ponytail Plugin Flag
ponytail_str = ""
flag_file = os.path.expanduser("~/.claude/.ponytail-active")
if os.path.isfile(flag_file):
    try:
        with open(flag_file) as f:
            mode = f.read().strip().upper() or "FULL"
            color = "173" if mode == "ULTRA" else "108"
            ponytail_str = f" {sep} \033[38;5;{color}m⚡ PONYTAIL: {mode}\033[0m"
    except Exception:
        pass

# Final Status Line output
print(f"\033[38;5;39m⚡ MODEL: {model_disp}\033[0m {sep} \033[38;5;220m📂 DIR: {short_cwd}\033[0m{git_str}{harness_str}{ponytail_str} {sep} \U0001f9e0 CTX: {ctx_color}{pct_int}%\033[0m")
