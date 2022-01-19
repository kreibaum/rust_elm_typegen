module Message exposing (..)


import Json.Decode
import Json.Encode
import Json.Decode.Pipeline

type alias Primitives =
    { a : Int
    , b : Int
    , c : Int
    , d : Int
    , e : Int
    , f : Int
    , g : Int
    , h : Int
    , i : Int
    , j : Int
    }

encodePrimitives : Primitives -> Json.Encode.Value
encodePrimitives primitives =
    Json.Encode.object
        [ ( "a", Json.Encode.int primitives.a )
        , ( "b", Json.Encode.int primitives.b )
        , ( "c", Json.Encode.int primitives.c )
        , ( "d", Json.Encode.int primitives.d )
        , ( "e", Json.Encode.int primitives.e )
        , ( "f", Json.Encode.int primitives.f )
        , ( "g", Json.Encode.int primitives.g )
        , ( "h", Json.Encode.int primitives.h )
        , ( "i", Json.Encode.int primitives.i )
        , ( "j", Json.Encode.int primitives.j )
        ]

decodePrimitives : Json.Decode.Decoder Primitives
decodePrimitives =
    Json.Decode.succeed Primitives
        |> Json.Decode.Pipeline.required "a" Json.Decode.int
        |> Json.Decode.Pipeline.required "b" Json.Decode.int
        |> Json.Decode.Pipeline.required "c" Json.Decode.int
        |> Json.Decode.Pipeline.required "d" Json.Decode.int
        |> Json.Decode.Pipeline.required "e" Json.Decode.int
        |> Json.Decode.Pipeline.required "f" Json.Decode.int
        |> Json.Decode.Pipeline.required "g" Json.Decode.int
        |> Json.Decode.Pipeline.required "h" Json.Decode.int
        |> Json.Decode.Pipeline.required "i" Json.Decode.int
        |> Json.Decode.Pipeline.required "j" Json.Decode.int
