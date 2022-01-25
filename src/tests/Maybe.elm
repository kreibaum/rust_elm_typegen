module Maybe exposing (..)


import Json.Decode
import Json.Encode
import Json.Decode.Pipeline

type alias ListMeMaybe =
    { listOfMaybe : (List (Maybe Int))
    , maybeOfList : (Maybe (List SomeDummyStruct))
    , doubleMaybe : (Maybe (Maybe Bool))
    }

encodeListMeMaybe : ListMeMaybe -> Json.Encode.Value
encodeListMeMaybe listmemaybe =
    Json.Encode.object
        [ ( "list_of_maybe", (Json.Encode.list (Maybe.map Json.Encode.int >> Maybe.withDefault Json.Encode.null)) listmemaybe.listOfMaybe )
        , ( "maybe_of_list", (Maybe.map (Json.Encode.list encodeSomeDummyStruct) >> Maybe.withDefault Json.Encode.null) listmemaybe.maybeOfList )
        , ( "double_maybe", (Maybe.map (Maybe.map Json.Encode.bool >> Maybe.withDefault Json.Encode.null) >> Maybe.withDefault Json.Encode.null) listmemaybe.doubleMaybe )
        ]

decodeListMeMaybe : Json.Decode.Decoder ListMeMaybe
decodeListMeMaybe =
    Json.Decode.succeed ListMeMaybe
        |> Json.Decode.Pipeline.required "list_of_maybe" (Json.Decode.list (Json.Decode.nullable Json.Decode.int))
        |> Json.Decode.Pipeline.required "maybe_of_list" (Json.Decode.nullable (Json.Decode.list decodeSomeDummyStruct))
        |> Json.Decode.Pipeline.required "double_maybe" (Json.Decode.nullable (Json.Decode.nullable Json.Decode.bool))

type alias SomeDummyStruct =
    { latitude : Int
    , longitude : Int
    }

encodeSomeDummyStruct : SomeDummyStruct -> Json.Encode.Value
encodeSomeDummyStruct somedummystruct =
    Json.Encode.object
        [ ( "latitude", Json.Encode.int somedummystruct.latitude )
        , ( "longitude", Json.Encode.int somedummystruct.longitude )
        ]

decodeSomeDummyStruct : Json.Decode.Decoder SomeDummyStruct
decodeSomeDummyStruct =
    Json.Decode.succeed SomeDummyStruct
        |> Json.Decode.Pipeline.required "latitude" Json.Decode.int
        |> Json.Decode.Pipeline.required "longitude" Json.Decode.int
