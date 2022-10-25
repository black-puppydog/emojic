// Code generated by github.com/orhanbalci/emojic/emojic-gen DO NOT EDIT.

// Source: https://unicode.org/Public/emoji/13.1/emoji-test.txt
// Created at: 2022-10-25 17:37:13.305947008 UTC
// TODO: change text below

#![allow(unused_imports)]

//! Flat list of all emojis without sub modules.
//!
//! This module contains the same set of emojis as the [`crate::grouped`] module, but
//! without the sub modules. This make it a bit more messy, but allows for shorter
//! references from code.
//!
//! # Examples
//!
//! ```rust
//! // prints: 🖼️
//! println!("{}", emojic::flat::FRAMED_PICTURE);
//! ```

pub const EMOJI_REGEX: &str = "🎨|🖼️|🪢|🎭|🪡|🧵|🧶|🥇|🎖️|🥈|🏅|🥉|🏆|🎟️|🎈|🎏|🎄|🎊|🧨|🎆|🎃|🎎|🎑|🎉|🎍|🧧|🎗️|🎀|🎇|✨|🎋|🎫|🎐|🎁|🎯|♟️|♣️|🔮|♦️|🎴|🎲|♥️|🃏|🕹️|🪁|🪄|🀄|🧿|🪆|🪅|🎱|🧩|🎰|♠️|🧸|🎮|🪀|🏈|🏸|⚾|🏀|🎳|🥊|🏏|🥌|🤿|🏑|🎣|⛳|🥏|🥅|🏒|⛸️|🥍|🥋|🏓|🏉|🎽|🎿|🛷|⚽|🥎|🎾|🏐|🐸|🐤|🐦|🐔|🦤|🕊️|🦆|🦅|🪶|🦩|🐥|🐣|🦉|🦜|🦚|🐧|🐓|🦢|🦃|🐜|🪲|🐛|🦋|🪳|🦗|🪰|🐝|🐞|🦠|🦟|🦂|🐌|🕷️|🕸️|🪱|🦡|🦇|🐻|🦫|🦬|🐈‍⬛|🐗|🐪|🐈|🐱|🐿️|🐄|🐮|🦌|🐕|🐶|🐘|🐑|🦊|🦒|🐐|🦍|🦮|🐹|🦔|🦛|🐎|🐴|🦘|🐨|🐆|🦁|🦙|🦣|🐒|🐵|🐁|🐭|🦧|🦦|🐂|🐼|🐾|🐖|🐷|🐽|🐻‍❄️|🐩|🐇|🐰|🦝|🐏|🐀|🦏|🐕‍🦺|🦨|🦥|🐅|🐯|🐫|🦄|🐃|🐺|🦓|🐡|🐬|🐟|🐙|🦭|🦈|🐚|🐳|🐠|🐋|🐊|🐉|🐲|🦎|🦕|🐍|🐢|🦖|🌼|💐|🌸|🌺|🌹|🏵️|🌻|🌷|💮|🥀|🌵|🌳|🌲|🍂|🍀|🌿|🍃|🍁|🌴|🪴|🌱|☘️|🌾|🦲|🦱|🦰|🦳|🏿|🏻|🏾|🏼|🏽|🇦🇫|🇦🇽|🇦🇱|🇩🇿|🇦🇸|🇦🇩|🇦🇴|🇦🇮|🇦🇶|🇦🇬|🇦🇷|🇦🇲|🇦🇼|🇦🇨|🇦🇺|🇦🇹|🇦🇿|🇧🇸|🇧🇭|🇧🇩|🇧🇧|🇧🇾|🇧🇪|🇧🇿|🇧🇯|🇧🇲|🇧🇹|🇧🇴|🇧🇦|🇧🇼|🇧🇻|🇧🇷|🇮🇴|🇻🇬|🇧🇳|🇧🇬|🇧🇫|🇧🇮|🇰🇭|🇨🇲|🇨🇦|🇮🇨|🇨🇻|🇧🇶|🇰🇾|🇨🇫|🇪🇦|🇹🇩|🇨🇱|🇨🇳|🇨🇽|🇨🇵|🇨🇨|🇨🇴|🇰🇲|🇨🇬|🇨🇩|🇨🇰|🇨🇷|🇨🇮|🇭🇷|🇨🇺|🇨🇼|🇨🇾|🇨🇿|🇩🇰|🇩🇬|🇩🇯|🇩🇲|🇩🇴|🇪🇨|🇪🇬|🇸🇻|🇬🇶|🇪🇷|🇪🇪|🇸🇿|🇪🇹|🇪🇺|🇫🇰|🇫🇴|🇫🇯|🇫🇮|🇫🇷|🇬🇫|🇵🇫|🇹🇫|🇬🇦|🇬🇲|🇬🇪|🇩🇪|🇬🇭|🇬🇮|🇬🇷|🇬🇱|🇬🇩|🇬🇵|🇬🇺|🇬🇹|🇬🇬|🇬🇳|🇬🇼|🇬🇾|🇭🇹|🇭🇲|🇭🇳|🇭🇰|🇭🇺|🇮🇸|🇮🇳|🇮🇩|🇮🇷|🇮🇶|🇮🇪|🇮🇲|🇮🇱|🇮🇹|🇯🇲|🇯🇵|🇯🇪|🇯🇴|🇰🇿|🇰🇪|🇰🇮|🇽🇰|🇰🇼|🇰🇬|🇱🇦|🇱🇻|🇱🇧|🇱🇸|🇱🇷|🇱🇾|🇱🇮|🇱🇹|🇱🇺|🇲🇴|🇲🇬|🇲🇼|🇲🇾|🇲🇻|🇲🇱|🇲🇹|🇲🇭|🇲🇶|🇲🇷|🇲🇺|🇾🇹|🇲🇽|🇫🇲|🇲🇩|🇲🇨|🇲🇳|🇲🇪|🇲🇸|🇲🇦|🇲🇿|🇲🇲|🇳🇦|🇳🇷|🇳🇵|🇳🇱|🇳🇨|🇳🇿|🇳🇮|🇳🇪|🇳🇬|🇳🇺|🇳🇫|🇲🇵|🇰🇵|🇲🇰|🇳🇴|🇴🇲|🇵🇰|🇵🇼|🇵🇸|🇵🇦|🇵🇬|🇵🇾|🇵🇪|🇵🇭|🇵🇳|🇵🇱|🇵🇹|🇵🇷|🇶🇦|🇷🇪|🇷🇴|🇷🇺|🇷🇼|🇼🇸|🇸🇲|🇸🇹|🇸🇦|🇸🇳|🇷🇸|🇸🇨|🇸🇱|🇸🇬|🇸🇽|🇸🇰|🇸🇮|🇸🇧|🇸🇴|🇿🇦|🇬🇸|🇰🇷|🇸🇸|🇪🇸|🇱🇰|🇧🇱|🇸🇭|🇰🇳|🇱🇨|🇲🇫|🇵🇲|🇻🇨|🇸🇩|🇸🇷|🇸🇯|🇸🇪|🇨🇭|🇸🇾|🇹🇼|🇹🇯|🇹🇿|🇹🇭|🇹🇱|🇹🇬|🇹🇰|🇹🇴|🇹🇹|🇹🇦|🇹🇳|🇹🇷|🇹🇲|🇹🇨|🇹🇻|🇺🇬|🇺🇦|🇦🇪|🇬🇧|🇺🇳|🇺🇸|🇺🇾|🇺🇲|🇻🇮|🇺🇿|🇻🇺|🇻🇦|🇻🇪|🇻🇳|🇼🇫|🇪🇭|🇾🇪|🇿🇲|🇿🇼|🏴|🏁|🎌|🏴‍☠️|🏳️‍🌈|🏳️‍⚧️|🚩|🏳️|🏴󠁧󠁢󠁥󠁮󠁧󠁿|🏴󠁧󠁢󠁳󠁣󠁴󠁿|🏴󠁧󠁢󠁷󠁬󠁳󠁿|🏺|🥢|🍴|🍽️|🔪|🥄|🍼|🍺|🧃|🍾|🧋|🍻|🥂|🍸|🥤|🥛|☕|🧊|🧉|🍶|🍵|🫖|🍹|🥃|🍷|🍱|🍚|🍛|🍡|🥟|🍥|🥠|🍤|🥮|🍢|🍙|🍘|🍠|🍝|🍜|🍣|🥡|🍌|🫐|🍒|🥥|🍇|🍏|🥝|🍋|🥭|🍈|🫒|🍑|🍐|🍍|🍎|🍓|🍊|🍅|🍉|🦀|🦞|🦪|🦐|🦑|🥓|🥯|🥖|🥣|🍞|🌯|🧈|🥫|🧀|🍳|🥐|🥩|🥚|🧆|🫓|🫕|🍟|🥗|🍔|🌭|🍖|🥞|🍕|🍿|🍲|🍗|🥨|🧂|🥪|🥘|🥙|🌮|🫔|🧇|🎂|🍬|🍫|🍪|🧁|🍮|🍩|🍯|🍨|🍭|🥧|🍧|🍰|🍦|🥑|🫑|🥦|🥕|🌰|🥒|🌽|🍆|🧄|🌶️|🥬|🍄|🧅|🥜|🥔|📘|🔖|📑|📚|📕|📗|🏷️|📒|📰|📓|📔|📖|📙|📄|📃|🗞️|📜|🎒|🩰|👙|🧢|🩲|👝|🧥|👑|👗|🥿|💎|👓|🧤|🥽|🎓|👜|👠|🥾|👖|👘|🥼|💄|👞|🪖|👔|🩱|📿|👛|⛑️|💍|👟|🦺|🥻|🧣|🛍️|🩳|🧦|🕶️|🩴|🎩|👕|👢|👚|👒|👡|🧮|🔋|💽|🖱️|🖥️|📀|🔌|💾|⌨️|💻|💿|🖨️|🖲️|🧺|🛁|🛏️|🧹|🪣|🪑|🛋️|🚪|🛗|🧯|🧴|🪞|🪤|🪠|🪒|🧻|🧷|🛒|🚿|🧼|🧽|🚽|🪥|🪟|📷|📸|🕯️|🎬|🪔|🎞️|📽️|🔦|💡|🔍|🔎|🎥|🏮|📺|📼|📹|🔑|🔒|🔐|🔏|🗝️|🔓|🗳️|📪|📫|✉️|📩|📧|📥|📨|📭|📬|📤|📦|📮|🩹|🩸|💊|🩺|💉|💹|🪙|💳|💵|💶|💰|💸|💷|🧾|💴|🎛️|🎧|🎚️|🎤|🎵|🎶|🎼|📻|🎙️|🪗|🪕|🥁|🎸|🪘|🎹|🎷|🎺|🎻|📊|💼|📅|🗃️|📇|🗂️|📉|📈|📋|🗄️|📁|🖇️|📂|📎|📌|📍|✂️|🗓️|🗒️|📏|📆|📐|🗑️|🚬|⚰️|⚱️|🪦|🗿|🪧|📠|📱|📲|📟|☎️|📞|⚗️|🧬|🔬|🧫|📡|🔭|🧪|🔔|🔕|📢|📣|🔇|📯|🔊|🔈|🔉|🪓|⚖️|🪃|🏹|🪚|⛓️|🗜️|⚔️|🗡️|⚙️|🔨|⚒️|🛠️|🪝|🪜|🔗|🧲|🔩|⛏️|🪛|🛡️|🧰|🔫|🦯|🔧|✒️|🖍️|🖋️|📝|🖌️|🖊️|✏️|🫀|🦴|🧠|👂|🦻|👁️|👀|💪|🦶|🦵|🫁|🦾|🦿|👄|👃|👅|🦷|💑|👪|💏|🧑‍🤝‍🧑|🤛|👊|✊|🤜|👎|👍|🖐️|🤚|✋|🖖|👋|🤙|🤞|🤟|👌|🤌|🤏|🤘|✌️|💅|🤳|✍️|👇|👈|👉|👆|☝️|🖕|👏|🙏|🤝|👐|🤲|🙌|👶|👦|🧒|👧|🧓|👵👴|🧑|🧗|💃🕺|💇|💆|🧑‍🦽|🧑‍🦼|🧖|🕴️|🧎|🏃|🧍|🚶|👯|🧑‍🦯|👼|🧝|🧚|🧞|🧙|🧜‍♀️|🧜‍♂️|🧜|🤶|🧑‍🎄|🎅|🦸|🦹|🧛|🧟|🧏|🙇|🤦|🙍|🙅|🙆|🙎|🙋|🤷|💁|🛌|🧘|🛀|🧑‍🎨|🧑‍🚀|🤱|👷|🧑‍🍳|🕵️|🧑‍🏭|🧑‍🌾|🧑‍🚒|💂|🧑‍⚕️|🧑‍⚖️|🧑‍🔧|🥷|🧑‍💼|🧑‍🍼|🤵|👳|👲|👰|🧑‍✈️|👮|🤰|🤴|👸|🧑‍🔬|🧑‍🎤|🧑‍🎓|🧑‍🏫|🧑‍💻|🧕|🏇|🚴|⛹️|🤸|🤺|🏌️|🤹|🏋️|🚵|🤾|🤽|🚣|🏄|🏊|🤼|⛷️|🏂|👥|👤|👣|🫂|🗣️|😹|😼|😿|😺|😸|😽|😾|😻|🙀|💢|💓|🖤|💙|💣|💔|🤎|💥|💨|💫|👁️‍🗨️|💚|💗|💟|❣️|❤️‍🔥|💘|💝|🕳️|💯|💋|🗨️|💌|❤️‍🩹|🧡|💜|❤️|💞|🗯️|💖|💬|💦|💭|💕|🤍|💛|💤|😘|😗|😚|😙|☺️|🥰|😍|🥲|🤩|😧|😰|😲|😖|😕|😢|😞|😓|😱|😮|😨|😳|☹️|😦|😯|😭|😣|🥺|😥|🙁|😫|😩|😟|🥱|👽|👾|🤡|👻|👺|👹|💩|🤖|🧐|🤓|😎|🤭|🤗|🤫|🤔|🤠|🥸|🥳|😠|👿|😤|🤬|😡|💀|☠️|😈|😑|😮‍💨|😶‍🌫️|😶|🤨|🙄|😬|🤥|😐|😏|😒|🤐|🤤|😔|😌|😴|😪|😁|😂|😀|😃|😄|😅|😆|🤣|🙂|😇|😊|🙃|😉|😋|😛|🤑|😝|😜|🤪|🥶|🤯|🤮|🤕|😷|😵‍💫|🤒|🥵|😵|🤢|🤧|🥴|🙉|🙈|🙊|🆎|🅰️|🅱️|Ⓜ️|🆑|🆒|🆓|🆔|ℹ️|🔤|🔡|🔠|🔢|🔣|🉑|🈸|🉐|㊗️|🈹|🈚|🈁|🈷️|🈶|🈵|🈺|🈴|🈲|🈯|㊙️|🈂️|🈳|🆕|🆖|🆗|🅾️|🅿️|🆘|🆙|🆚|🔙|🔃|🔄|⬇️|↙️|↘️|🔚|⬅️|↪️|↔️|🔛|➡️|⤵️|↩️|⤴️|🔜|🔝|⬆️|↕️|↖️|↗️|📶|🔆|🎦|🔅|🔽|⏏️|⏬|⏩|⏪|⏫|⏮️|📴|⏭️|⏸️|▶️|⏯️|⏺️|🔁|🔂|◀️|🔀|⏹️|🔼|📳|💱|💲|♀️|♂️|⚧️|⚫|⬛|◾|◼️|▪️|🔲|🔵|🟦|🟤|🟫|💠|🟢|🟩|🔷|🔶|🟠|🟧|🟣|🟪|🔘|🔴|🟥|🔻|🔺|🔹|🔸|⚪|⬜|◽|◻️|▫️|🔳|🟡|🟨|0️⃣|1️⃣|🔟|2️⃣|3️⃣|4️⃣|5️⃣|6️⃣|7️⃣|8️⃣|9️⃣|*️⃣|#️⃣|➗|♾️|➖|✖️|➕|☑️|✔️|✅|©️|❌|❎|➰|➿|✴️|✳️|⚜️|⭕|🔰|⚕️|📛|〽️|♻️|®️|❇️|™️|🔱|‼️|⁉️|❗|❓|〰️|❕|❔|⚛️|🔯|✝️|🕎|🕉️|☦️|☮️|🛐|☪️|✡️|☸️|☯️|🏧|🚼|🛄|🛃|🛅|🚮|🚹|🛂|🚰|🚻|🚾|♿|🚺|☣️|🚸|🚱|🚳|⛔|🚯|📵|🔞|🚷|🚭|🚫|☢️|⚠️|♒|♈|♋|♑|♊|♌|♎|⛎|♓|♐|♏|♉|♍|🛎️|🧳|🏦|🧱|🏗️|🏰|🏛️|🏪|🏬|🏚️|🏭|🏥|🏨|🏠|🏘️|🏡|🛖|🏯|🏣|🏩|🏢|🏤|🪨|🏫|🏟️|🗽|🗼|💒|🪵|🏖️|🏕️|🏜️|🏝️|⛰️|🗻|🏞️|🏔️|🌋|🧭|🌎|🌏|🌍|🌐|🗾|🗺️|💈|🌉|🎠|🎪|🏙️|🌆|🎡|🌁|⛲|♨️|🌃|🎢|🌅|🌄|🌇|⛺|⛪|🛕|🕋|🕌|⛩️|🕍|🌂|☁️|🌩️|⛈️|🌧️|🌨️|☄️|🌙|🌀|💧|🔥|🌓|🌛|🌫️|🌕|🌝|🌟|⚡|🌗|🌜|🌌|🌑|🌚|🌈|🪐|🌠|❄️|☃️|⛄|⭐|☀️|⛅|🌥️|🌦️|🌤️|🌞|🌡️|🌪️|☂️|⛱️|☔|🌘|🌖|🌊|🌒|🌔|🌬️|⏰|🕗|🕣|🕚|🕦|🕔|🕠|🕓|🕟|⌛|⏳|🕰️|🕘|🕤|🕐|🕜|🕖|🕢|🕕|🕡|⏱️|🕙|🕥|🕒|🕞|⏲️|🕛|🕧|🕑|🕝|⌚|🚡|✈️|🛬|🛫|🛸|🚁|🚠|🪂|🚀|🛰️|💺|🛩️|🚟|🚑|🚛|🚗|🛺|🚲|🚅|🚌|🚏|🚧|🚚|🚒|⛽|🚄|🚥|🛴|🚈|🚂|🦽|🚇|🚐|🚝|🏍️|🦼|🛣️|🛵|🚞|🛢️|🚘|🚍|🚔|🚖|🛻|🚓|🚨|🏎️|🚃|🛤️|🛼|🛹|🚙|🚉|🛑|🚕|🚜|🚆|🚊|🚋|🚎|🚦|⚓|🛶|⛴️|🛥️|🛳️|⛵|🚢|🚤";
