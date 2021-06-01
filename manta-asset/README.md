Manta Asset
------

This crate implements the data types for manta private asset.

There are three structures related to the private asset.

1. `MantaAsset` is the struct for the asset. It is the data type that one may generate
if they mint the private asset themselves.

2. `MantaAssetFullReceiver` is the struct build by a receivers, i.e., 
someone who wishes to receiver private
assets from others generates. It contains necessary address related information, 
`MantaAssetPreparedReceiver`, so that an sender can send the private assets to; also 
additional information, `MantaAssetReceiverSpendingInfo`, that allows for spending this private
assets once the transaction is completed. 

3. `MantaAssetProcessedReceiver` is the struct that build by the sender. The 
sender, after gathering `MantaAssetPreparedReceiver`, will convert the receiver to
a processed one, so that the output `MantaAssetProcessedReceiver` is readable by
the chain.


The crate also implements the following traits

1. `sampling`: sampling random `MantaAsset` and `MantaAssetFullReceiver`
2. `processing`: converting `MantaAssetPreparedReceiver` to `MantaAssetProcessedReceiver`
3. `serdes`: (de)serialization methods for above structs.
