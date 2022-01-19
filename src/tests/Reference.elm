module Message exposing (..)


import Json.Decode
import Json.Encode
import Json.Decode.Pipeline

type alias WeatherData =
    { position : Coordinate
    , temperature : Int
    , humidity : Int
    }

encodeWeatherData : WeatherData -> Json.Encode.Value
encodeWeatherData weatherdata =
    Json.Encode.object
        [ ( "position", encodeCoordinate weatherdata.position )
        , ( "temperature", Json.Encode.int weatherdata.temperature )
        , ( "humidity", Json.Encode.int weatherdata.humidity )
        ]

decodeWeatherData : Json.Decode.Decoder WeatherData
decodeWeatherData =
    Json.Decode.succeed WeatherData
        |> Json.Decode.Pipeline.required "position" decodeCoordinate
        |> Json.Decode.Pipeline.required "temperature" Json.Decode.int
        |> Json.Decode.Pipeline.required "humidity" Json.Decode.int

type alias Coordinate =
    { latitude : Int
    , longitude : Int
    }

encodeCoordinate : Coordinate -> Json.Encode.Value
encodeCoordinate coordinate =
    Json.Encode.object
        [ ( "latitude", Json.Encode.int coordinate.latitude )
        , ( "longitude", Json.Encode.int coordinate.longitude )
        ]

decodeCoordinate : Json.Decode.Decoder Coordinate
decodeCoordinate =
    Json.Decode.succeed Coordinate
        |> Json.Decode.Pipeline.required "latitude" Json.Decode.int
        |> Json.Decode.Pipeline.required "longitude" Json.Decode.int
