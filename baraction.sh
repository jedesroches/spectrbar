#!/bin/sh
# Program to output useful information to spectrwm's status bar.
# Written at home by sparrowhawk, anno domini 2021.

BATT_LOW_WARN=15
REFRESH_RATE=5
ROOT_FULL_WARN=85

# Right-pad field with a space if non-empty.
build_field()
{
	if [ -n "$(printf "%s" "$1" | tr -d '[:space:]')" ];
	then
		printf "%s " "$1"
	fi
}

# Right-pad with a | if non-empty.
build_section()
{
	if [ -n "$(printf "%s" "$1" | tr -d '[:space:]')" ];
	then
		printf "%.38s| " "$1"
	fi
}

get_batt()
{
	cap=$(cat "/sys/class/power_supply/BAT$1/capacity")
	if [ "$cap" -le $BATT_LOW_WARN ];
	then
		printf "+@fg=1;%s+@fg=0; " "$cap"
	else
		printf "%s " "$cap"
	fi
}

get_batts()
{
	build_section "$(get_charg)$(get_batt 0)$(get_batt 1)"
}

get_charg()
{
	charging=$(cat "/sys/class/power_supply/AC/online")
	if [ "$charging" -eq 1 ];
	then
		printf "%s " '⚇';
	fi
}

get_enp()
{
	if ip link show | grep -qE '(enp.*).*(,UP,)';
	then
		printf "eth "
	fi
}

# Get string to display for an email count.
get_mail()
{
	msgs="$(find "$HOME/.local/var/mail/$1/Inbox/new" \
		-maxdepth 1 -type f -printf x | wc -c)"
	if [ "$msgs" -ne 0 ];
	then
		printf "%s: %d " "$1" "$msgs"
	fi
}

get_mails()
{
	MAILS=
	for m in 'kleis' 'thedesroches' 'gmail';
	do
		MAILS="${MAILS}$(get_mail $m)"
	done
	build_section "$MAILS"
}

get_mpd()
{
	if [ "$(mpc status '%state%')" = "playing" ];
	then
		build_section "$(mpc current -f '%artist% - %title%') "
	fi
}

get_networks()
{
	build_section "$(get_enp)$(get_wifi)"
}

get_root()
{
	if [ "$(df / --output=pcent | grep -o '[0-9]*')" -ge $ROOT_FULL_WARN ];
	then
		build_section "+@fg=1;WARN: ROOT FULL+@fg=0; "
	fi
}

get_wifi()
{
	# shellcheck disable=SC2016
	awk_code='/Connected network/ { for (i=3; i<=NF; i++) printf("%s", $i) }'
	build_field \
		"$(iwctl station wlan0 show | awk "$awk_code")"
}

while :;
do

	printf "%s%s%s%s%s\n" "$(get_mpd)" "$(get_root)" "$(get_mails)" "$(get_networks)" "$(get_batts)"
	sleep $REFRESH_RATE
done
