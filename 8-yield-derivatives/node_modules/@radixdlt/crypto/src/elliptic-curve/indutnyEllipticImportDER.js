// @ts-nocheck
/* eslint-disable */
var BN = require('bn.js')

// from: https://github.com/indutny/minimalistic-crypto-utils
// and
// https://github.com/indutny/elliptic/blob/master/lib/elliptic/utils.js
// copyright: Fedor Indutny
// https://github.com/indutny
// and
// https://github.com/indutny/elliptic/blob/master/lib/elliptic/ec/signature.js

function Position() {
	this.place = 0
}

function getLength(buf, p) {
	var initial = buf[p.place++]
	if (!(initial & 0x80)) {
		return initial
	}
	var octetLen = initial & 0xf

	// Indefinite length or overflow
	if (octetLen === 0 || octetLen > 4) {
		return false
	}

	var val = 0
	for (var i = 0, off = p.place; i < octetLen; i++, off++) {
		val <<= 8
		val |= buf[off]
		val >>>= 0
	}

	// Leading zeroes
	if (val <= 0x7f) {
		return false
	}

	p.place = off
	return val
}

function rmPadding(buf) {
	var i = 0
	var len = buf.length - 1
	while (!buf[i] && !(buf[i + 1] & 0x80) && i < len) {
		i++
	}
	if (i === 0) {
		return buf
	}
	return buf.slice(i)
}

function toArray(msg, enc) {
	if (Array.isArray(msg)) return msg.slice()
	if (!msg) return []
	var res = []
	if (typeof msg !== 'string') {
		for (var i = 0; i < msg.length; i++) res[i] = msg[i] | 0
		return res
	}
	if (enc === 'hex') {
		msg = msg.replace(/[^a-z0-9]+/gi, '')
		if (msg.length % 2 !== 0) msg = '0' + msg
		for (var i = 0; i < msg.length; i += 2)
			res.push(parseInt(msg[i] + msg[i + 1], 16))
	} else {
		for (var i = 0; i < msg.length; i++) {
			var c = msg.charCodeAt(i)
			var hi = c >> 8
			var lo = c & 0xff
			if (hi) res.push(hi, lo)
			else res.push(lo)
		}
	}
	return res
}

function zero2(word) {
	if (word.length === 1) return '0' + word
	else return word
}

function toHex(msg) {
	var res = ''
	for (var i = 0; i < msg.length; i++) res += zero2(msg[i].toString(16))
	return res
}

function encode(arr, enc) {
	if (enc === 'hex') return toHex(arr)
	else return arr
}

function __js_importDER(data, enc) {
	data = toArray(data, enc)
	var p = new Position()
	if (data[p.place++] !== 0x30) {
		return null
	}
	var len = getLength(data, p)
	if (len === false) {
		return null
	}
	if (len + p.place !== data.length) {
		return null
	}
	if (data[p.place++] !== 0x02) {
		return null
	}
	var rlen = getLength(data, p)
	if (rlen === false) {
		return null
	}
	var r = data.slice(p.place, rlen + p.place)
	p.place += rlen
	if (data[p.place++] !== 0x02) {
		return null
	}
	var slen = getLength(data, p)
	if (slen === false) {
		return null
	}
	if (data.length !== slen + p.place) {
		return null
	}
	var s = data.slice(p.place, slen + p.place)
	if (r[0] === 0) {
		if (r[1] & 0x80) {
			r = r.slice(1)
		} else {
			// Leading zeroes
			return null
		}
	}
	if (s[0] === 0) {
		if (s[1] & 0x80) {
			s = s.slice(1)
		} else {
			// Leading zeroes
			return null
		}
	}

	return {
		r: new BN(r),
		s: new BN(s),
	}
}

function constructLength(arr, len) {
	if (len < 0x80) {
		arr.push(len)
		return
	}
	var octets = 1 + ((Math.log(len) / Math.LN2) >>> 3)
	arr.push(octets | 0x80)
	while (--octets) {
		arr.push((len >>> (octets << 3)) & 0xff)
	}
	arr.push(len)
}

function __js_toDER(r, s, enc) {
	var r = r.toArray()
	var s = s.toArray()

	// Pad values
	if (r[0] & 0x80) r = [0].concat(r)
	// Pad values
	if (s[0] & 0x80) s = [0].concat(s)

	r = rmPadding(r)
	s = rmPadding(s)

	while (!s[0] && !(s[1] & 0x80)) {
		s = s.slice(1)
	}
	var arr = [0x02]
	constructLength(arr, r.length)
	arr = arr.concat(r)
	arr.push(0x02)
	constructLength(arr, s.length)
	var backHalf = arr.concat(s)
	var res = [0x30]
	constructLength(res, backHalf.length)
	res = res.concat(backHalf)
	return encode(res, enc)
}

module.exports = {
	__js_importDER,
	__js_toDER,
}
/* eslint-enable */
