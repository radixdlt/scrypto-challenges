This exchange is based of an extention of the "buy me buy you" principle so it can be used to change the number of owners in a firm or a DAO, for any size of firm. Albeit, by the nature of the algorithm it targets smaller/private firms since for these companies there is a limited number of valuators, and thus a lack of an objective price.
The package is based on the solution from this paper:
https://papers.ssrn.com/sol3/papers.cfm?abstract_id=4061300

Unfortunately, I noticed the challenge at the last minute, so I didn't get to do all the necessary debugging by the deadline.
It's exciting to be a part of this so I'm submitting anyway.

Usage (after debugging):
1) By instantiating the component the caller gets 100 shares (resource). 
He/she distributes or sell the shares in any way they like, and then there are multiple owners.
2) Every owner submits (by method place_trade) his/her shares together with enough XRD to buy the shares of anyone who might sell, such that the value he/she has for a share is derived from the amount of XRD needed to buy all the shares and the total number of shares.
3) The exchange picks who becomes a buyer and who becomes a seller (by method redistribute).
All the buyers gets to buy shares at a price lower than they value them, and all the sellers are payed more than they value the shares.


## License

The Radix Scrypto Challenges code is released under Radix Modified MIT License.

    Copyright 2024 Radix Publishing Ltd

    Permission is hereby granted, free of charge, to any person obtaining a copy of
    this software and associated documentation files (the "Software"), to deal in
    the Software for non-production informational and educational purposes without
    restriction, including without limitation the rights to use, copy, modify,
    merge, publish, distribute, sublicense, and to permit persons to whom the
    Software is furnished to do so, subject to the following conditions:

    This notice shall be included in all copies or substantial portions of the
    Software.

    THE SOFTWARE HAS BEEN CREATED AND IS PROVIDED FOR NON-PRODUCTION, INFORMATIONAL
    AND EDUCATIONAL PURPOSES ONLY.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
    FOR A PARTICULAR PURPOSE, ERROR-FREE PERFORMANCE AND NONINFRINGEMENT. IN NO
    EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES,
    COSTS OR OTHER LIABILITY OF ANY NATURE WHATSOEVER, WHETHER IN AN ACTION OF
    CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
    SOFTWARE OR THE USE, MISUSE OR OTHER DEALINGS IN THE SOFTWARE. THE AUTHORS SHALL
    OWE NO DUTY OF CARE OR FIDUCIARY DUTIES TO USERS OF THE SOFTWARE.