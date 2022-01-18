module Person exposing (..)


import Json.Decode
import Json.Encode
import Json.Decode.Pipeline

type alias Person =
    { age : Int
    , surname : String
    }

encodePerson : Person -> Json.Encode.Value
encodePerson person =
    Json.Encode.object
        [ ( "age", Json.Encode.int person.age )
        , ( "surname", Json.Encode.string person.surname )
        ]

decodePerson : Json.Decode.Decoder Person
decodePerson =
    Json.Decode.succeed Person
        |> Json.Decode.Pipeline.required "age" Json.Decode.int
        |> Json.Decode.Pipeline.required "surname" Json.Decode.string
