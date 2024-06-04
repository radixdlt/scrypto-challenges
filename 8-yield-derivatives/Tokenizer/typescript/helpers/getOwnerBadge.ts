export const getOwnerBadge = (
  events: any
): {
  ownerBadgeAddress: string
  ownerBadgeResourceAddress: string
  ownerBadgeNonFungibleId: string
} => {
  const nftDepositEvent = events.find(
    (event: any) =>
      event.name === 'DepositEvent' && event.data.variant_name === 'NonFungible'
  )!

  const {
    data: { fields },
  } = nftDepositEvent

  const ownerBadgeResourceAddress: string = fields.find(
    (field: any) => field.type_name === 'ResourceAddress'
  )!.value

  const ownerBadgeNonFungibleId: string = fields.find(
    (field: any) => field.element_kind === 'NonFungibleLocalId'
  )!.elements[0].value

  const ownerBadgeAddress = `${ownerBadgeResourceAddress}:${ownerBadgeNonFungibleId}`

  return {
    ownerBadgeAddress,
    ownerBadgeNonFungibleId,
    ownerBadgeResourceAddress,
  }
}
