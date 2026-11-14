# Approved public source capture

Representation: full web-reader page extraction, preserved below as returned. Direct raw-HTML downloads returned HTTP 429; no retry loop, login, browser or provider API was used. This is primary public-page content, not proof of Metten adapter capability. Dynamic content absent from the extraction remains unverified.

15-minute markets · Help · Kalshi Pro (https://kalshi.com/pro/help/fifteen-minute-markets)
citeturn1view1 [wordlim: 200] Crawled: today; Content type: text/html; Source: open({"ref_id":"https://kalshi.com/pro/help/fifteen-minute-markets","lineno":null}); Total lines: 128
L0: Help Center
L1: # How can we help?
L2: 
L3: Everything you need to trade on Kalshi Pro, from your first fill to leverage on perpetuals.
L4: 
L5: [Input: Search articles]
L6: 
L7:   * Start here
L8: 
L9:     * 【0†Get started with Kalshi Pro】
L10:     * 【1†Recommended specs & browser support】
L11:   * Find markets
L12: 
L13:     * 【2†Markets: scanning, filtering, and searching】
L14:     * 【3†Charts on Kalshi Pro】
L15:   * Build your workspace
L16: 
L17:     * 【4†Canvas: your multi-market workspace】
L18:     * 【5†Canvas views and search】
L19:   * Trade
L20:     * 【6†The order panel】
L21:     * 【7†Order book views】
L22:     * 【8†Managing orders and positions】
L23:     * 【9†Perpetuals on Kalshi Pro (Beta)】
L24:     * 【10†15-minute markets】
L25:   * Reference
L26: 
L27:     * 【11†Trading glossary and terminology】
L28:     * 【12†Settings and preferences】
L29:     * 【13†Default order behaviors and settings】
L30: 
L31: Trade
L32: # 15-minute markets
L33: 
L34: Built for markets that settle every 15 minutes: see where the price stands against the target, act from one screen, and roll into the next window without reopening anything.
L35: 
L36: 5 min read · Updated Sep 28, 2026
L37: 
L38: * * *
L39: A 15-minute market opens, trades, and settles in about the time it takes to read this page. The underlying can cross the target several times in one window, and the book reprices every time it does. With so little time on the clock, the seconds you spend switching screens or reopening a market are seconds you’re not trading.
L40: The 15-minute page is built for that pace. Every 15-minute series, like 15-minute Bitcoin, gets one screen with everything you need to decide and act: where the price stands against the target and how long is left, a chart that updates by the second, the live order book, the order panel, and your positions. When a window closes, the next one is already loaded, so you can trade a series window after window without losing your place.
L41: 【14†Image: The 15-minute Bitcoin market page: target, current price, and countdown across the top, the price chart with a position and a resting order drawn on it, the order book, and the order panel on the right.†pro.kalshi.com】
L42: 
L43: The 15-minute page: header stats, chart, order book, and order panel on one screen.
L44: ## Opening the page
L45: 
L46: Wherever a 15-minute market appears, look for the Open full page button:
L47: 
L48:   * •Markets: on the market’s row, next to its name.
L49:   * •Trade window: in the header, beside the other actions.
L50:   * •Canvas: in the tile’s header, shown below.
L51: 
L52: 【15†Image: The header of a 15-minute Bitcoin Canvas tile, with the Open full page icon (a box with an arrow pointing into it) hovered to show its tooltip.†pro.kalshi.com】
L53: 
L54: The Open full page icon on a Canvas tile.
L55: To move to another 15-minute series, click the market name at the top of the page. The picker lists every active series with its target, current price, distance from target, Yes price, spread, and volume, and you can sort any column or search.
L56: 
L57: 【16†Image: The 15-minute market picker: six series with target, current price, distance from target, Yes price, spread, and volume, sorted by distance from target and then searched for gold.†pro.kalshi.com】Pause
L58: Sort the picker by any column, or search for a series.
L59: ## It always shows the live market
L60: 
L61: The page belongs to the series, not to a single market. When the current window closes, the page moves to the next one on its own: the chart, book, order panel, and blotter all switch in place. Your chart interval and indicators carry over; drawings start fresh with each market. The address never changes, so a bookmark always opens whatever is trading now.
L62: When a market you watched live settles, a notification shows the result (for example, “Settled Up”) and, when available, the final price against the target, so you know how the last window went without leaving the next one. The Alerts switch in the notifications menu turns these on or off.
L63: ## Reviewing a past market
L64: 
L65: Click an earlier market in the blotter and the page opens it and stays there instead of rolling forward. A banner names the window you’re looking at (“Viewing a past slot · Mon, Sep 22, 4:00 PM ET”); click Go to live market to return to the live market. Sharing the page’s link while you’re on a past market opens that same market for whoever follows it.
L66: 【17†Image: A settled 15-minute market with its Settled Yes badge and a Viewing a past slot banner; clicking Go to live market switches the page to the live market.†pro.kalshi.com】Pause
L67: 
L68: Go to live market takes you back to the window trading now.
L69: ## Chart views
L70: 
L71: The chart has four views, one tab apart. Pick one to see it.
L72: 
L73: 【18†Image: The BTC index cycling from a pane below volume to an overlay with a left-hand scale, then hidden and back to a pane. Compare symbol adds ETH’s 15m Yes prices to the trade chart.†pro.kalshi.com】Pause
L74: 
L75: ‹›
L76: 
L77: 1 / 4
L78:   * 【18†Image†pro.kalshi.com】Price: the market’s trades, down to one-second bars, for whichever side, Yes or No, the order panel is on. Where available, the underlying index starts in a pane below volume. Its header button cycles through pane, overlay on the trades with a left-hand price scale, and hidden. Use Compare symbol (+) to add another market’s 15m Yes prices, index, or TWAP when available.
L79:   * 【19†Image†pro.kalshi.com】Line: the underlying’s price racing its target in real time, the same knockout chart Canvas tiles use.
L80:   * 【20†Image†pro.kalshi.com】Index: zoom out from the window in front of you. Candles of the index the market settles on, from one minute to one day, with the target marked, show the trend this window sits inside.
L81:   * 【21†Image†pro.kalshi.com】Depth: the cumulative order book for the side you’re trading. Hover over it for the depth at any price, and scroll to zoom in or out around the midpoint.
L82: Fifteen minutes is a slice of a longer move. Before you trade a window, open Index and step back to hourly or daily candles: whether the target sits at the edge of the day’s range or against a strong trend is context the Price chart alone won’t show you.
L83: ## Order book and trades
L84: 
L85: The column beside the chart holds the order book and every trade as it prints. Set it up the way you read the market; Pro remembers your choices.
L86: 
L87: 【22†Image: The order book’s ⋮ menu switching from Tab to Stacked, with trade history under the book, then to Large, with the book and trade history side by side.†pro.kalshi.com】Pause
L88: 
L89: ‹›
L90: 
L91: 1 / 4
L92:   * 【22†Image†pro.kalshi.com】Layout: from the ⋮ menu, Tab keeps the book and trade history behind tabs in one column, Stacked puts the book above the trades, and Large sets them side by side at double width.
L93:   * 【23†Image†pro.kalshi.com】Book view: 1 side follows the side you’re trading, 2 sides shows separate Yes and No ladders, Maker lists the Yes and No bids, and Taker lists the Yes and No asks.
L94:   * 【24†Image†pro.kalshi.com】Totals and precision: show the running total in contracts or dollars, and quantities to two decimals or whole contracts.
L95:   * 【25†Image†pro.kalshi.com】Trade history: every print in this market, newest first, with the side taken, price, size, and time.
L96: Clicking a price in the book fills the order panel. Keep side keeps your Buy or Sell selection; Take liquidity buys from an ask and, when you own shares, sells into a bid.
L97: ## Trading from the chart
L98: 
L99: Your orders and position are drawn right on the Price chart, and you can trade from them without leaving it. Pick a feature to see it in action.
L100: 
L101: 【26†Image: The + on the price chart’s axis at 53¢ opening a Buy Yes limit order, with 10 contracts typed in.†pro.kalshi.com】Pause
L102: 
L103: ‹›
L104: 
L105: 1 / 5
L106:   * 【26†Image†pro.kalshi.com】Place an order: hover the price axis and click + to open a limit buy at that price, then size it in contracts or dollars.
L107:   * 【27†Image†pro.kalshi.com】Move an order: your resting orders are drawn as lines. Drag one to a new price to amend it.
L108:   * 【28†Image†pro.kalshi.com】Cancel an order: click the × on an order’s label to cancel it.
L109:   * 【29†Image†pro.kalshi.com】Close or reverse a position: your position is a line at its average price. Its label opens a ticket to close it or flip it to the other side.
L110:   * 【30†Image†pro.kalshi.com】See your fills: your buys and sells are marked B and S on the chart. Hover over one for its size and price.
L111: Turn the lines and marks on or off from the settings (gear) button on the chart.
L112: ## The blotter
L113: 
L114: The blotter covers the whole series, so your fills and positions from earlier windows stay put after a rollover. Alongside Positions, Orders, and Fills, a Settled tab lists each closed market, how it resolved, and your payout and realized P&L. Drag the divider above it to trade height between the chart and the blotter; Pro remembers the split.
L115: 【31†Image: The blotter under the chart switching from Positions to Settled, which lists the 4:00 PM market that settled Yes with its payout and realized P&L.†pro.kalshi.com】Pause
L116: 
L117: The Settled tab: every closed window in the series and how it paid out.
L118: 
L119: 【32†Open Kalshi Pro†pro.kalshi.com】【33†Help Center】【11†Glossary】【34†What’s new】
L120: 
L121: 【0†Getting started】【6†Place a trade】【9†Perpetuals】
L122: 
L123: 【35†kalshi.com】【36†Member Agreement】【37†Privacy】
L124: Perpetual futures involve substantial risk of loss and are not suitable for all customers; you may lose all funds you commit, including fees and commissions, and you may be required to post additional margin or make additional payments. 【38†See disclosures】.
L125: © 2026 Kalshi. Restrictions and eligibility requirements apply. Trading on Kalshi involves risk and may not be appropriate for all. Members risk losing their cost to enter any transaction, including fees. You should carefully consider whether trading on Kalshi is appropriate for you in light of your investment experience and financial resources. Any trading decisions you make are solely your responsibility and at your own risk. Information is provided for convenience only on an “AS IS” basis.
L126: Past performance is not necessarily indicative of future results. Kalshi products are not available in all jurisdictions. See 【39†kalshi.com/regulatory】 for more information. Kalshi is subject to U.S. regulatory oversight by the CFTC.
L127: By using Kalshi Pro you acknowledge and agree to Kalshi’s [Button: legal terms].


