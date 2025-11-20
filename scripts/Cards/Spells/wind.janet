(def wind @{
'cost 2
'on-play  @[(table
  'action (fn [game card-id] (std/apply-effect game 'weakening 2 (std/from-current-position game card-id (std/plus 1))))
  'timing @['now]
  'targeting ['single]
  )]

'description "Blows away a card"

'display-image-asset-string "missing"
})
