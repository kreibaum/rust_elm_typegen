module Message exposing (..)


import Json.Decode
import Json.Encode
import Json.Decode.Pipeline

type RemoteMessage
    = Hello String
    | Compare Int Int
    | Juggle Int String String
    | Goodbye


decodeRemoteMessage : Json.Decode.Decoder RemoteMessage
decodeRemoteMessage =
    Json.Decode.oneOf
        [ decodeRemoteMessageHello
        , decodeRemoteMessageCompare
        , decodeRemoteMessageJuggle
        , decodeRemoteMessageGoodbye
        ]


decodeRemoteMessageHello : Json.Decode.Decoder RemoteMessage
decodeRemoteMessageHello =
    Json.Decode.succeed Hello
        |> Json.Decode.Pipeline.required "Hello" Json.Decode.string


decodeRemoteMessageCompare : Json.Decode.Decoder RemoteMessage
decodeRemoteMessageCompare =
    Json.Decode.succeed Compare
        |> Json.Decode.Pipeline.custom 
            (Json.Decode.field "Compare" (Json.Decode.index 0 Json.Decode.int))
        |> Json.Decode.Pipeline.custom 
            (Json.Decode.field "Compare" (Json.Decode.index 1 Json.Decode.int))


decodeRemoteMessageJuggle : Json.Decode.Decoder RemoteMessage
decodeRemoteMessageJuggle =
    Json.Decode.succeed Juggle
        |> Json.Decode.Pipeline.custom 
            (Json.Decode.field "Juggle" (Json.Decode.index 0 Json.Decode.int))
        |> Json.Decode.Pipeline.custom 
            (Json.Decode.field "Juggle" (Json.Decode.index 1 Json.Decode.string))
        |> Json.Decode.Pipeline.custom 
            (Json.Decode.field "Juggle" (Json.Decode.index 2 Json.Decode.string))


decodeRemoteMessageGoodbye : Json.Decode.Decoder RemoteMessage
decodeRemoteMessageGoodbye =
    Json.Decode.andThen
        (\str ->
            case str of
                "Goodbye" ->
                    Json.Decode.succeed Goodbye

                _ ->
                    Json.Decode.fail "Expected variant Goodbye"
        )
        Json.Decode.string
