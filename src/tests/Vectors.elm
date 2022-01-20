module Vectors exposing (..)


import Json.Decode
import Json.Encode
import Json.Decode.Pipeline

type alias Card =
    { suit : String
    , value : Int
    }

encodeCard : Card -> Json.Encode.Value
encodeCard card =
    Json.Encode.object
        [ ( "suit", Json.Encode.string card.suit )
        , ( "value", Json.Encode.int card.value )
        ]

decodeCard : Json.Decode.Decoder Card
decodeCard =
    Json.Decode.succeed Card
        |> Json.Decode.Pipeline.required "suit" Json.Decode.string
        |> Json.Decode.Pipeline.required "value" Json.Decode.int

type alias GameState =
    { deck : (List Card)
    , discardPile : (List Card)
    , replayHistory : (List Action)
    }

encodeGameState : GameState -> Json.Encode.Value
encodeGameState gamestate =
    Json.Encode.object
        [ ( "deck", (Json.Encode.list encodeCard) gamestate.deck )
        , ( "discard_pile", (Json.Encode.list encodeCard) gamestate.discardPile )
        , ( "replay_history", (Json.Encode.list encodeAction) gamestate.replayHistory )
        ]

decodeGameState : Json.Decode.Decoder GameState
decodeGameState =
    Json.Decode.succeed GameState
        |> Json.Decode.Pipeline.required "deck" (Json.Decode.list decodeCard)
        |> Json.Decode.Pipeline.required "discard_pile" (Json.Decode.list decodeCard)
        |> Json.Decode.Pipeline.required "replay_history" (Json.Decode.list decodeAction)

type Action
    = PlayCard Card
    | DiscardCards (List Card)
    | Surrender

encodeAction : Action -> Json.Encode.Value
encodeAction action =
    case action of
        PlayCard x ->
            Json.Encode.object
                [ ( "PlayCard", encodeCard x )
                ]

        DiscardCards x ->
            Json.Encode.object
                [ ( "DiscardCards", (Json.Encode.list encodeCard) x )
                ]

        Surrender ->
            Json.Encode.string "Surrender"


decodeAction : Json.Decode.Decoder Action
decodeAction =
    Json.Decode.oneOf
        [ decodeActionPlayCard
        , decodeActionDiscardCards
        , decodeActionSurrender
        ]


decodeActionPlayCard : Json.Decode.Decoder Action
decodeActionPlayCard =
    Json.Decode.succeed PlayCard
        |> Json.Decode.Pipeline.required "PlayCard" decodeCard


decodeActionDiscardCards : Json.Decode.Decoder Action
decodeActionDiscardCards =
    Json.Decode.succeed DiscardCards
        |> Json.Decode.Pipeline.required "DiscardCards" (Json.Decode.list decodeCard)


decodeActionSurrender : Json.Decode.Decoder Action
decodeActionSurrender =
    Json.Decode.andThen
        (\str ->
            case str of
                "Surrender" ->
                    Json.Decode.succeed Surrender

                _ ->
                    Json.Decode.fail "Expected variant Surrender"
        )
        Json.Decode.string
