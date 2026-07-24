param([switch]$Generate)
$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
$workspace=Join-Path $root 'morphospace'
$receiptPath=Join-Path $workspace 'receipts/rlsl-lslc-004q-historical-unit-adoption.json'
$statePath=Join-Path $workspace 'workspace.state.json'
$ownerCommit='50f8e8a67641f535347c3061d531e6d4df46e535'
$ownerRoot=$env:RLSL_WORK_ENVIRONMENT_ROOT
$currentCategories=@('implementation','authority','module-layout','feature-activation','validation','device-policy','repo-routing','public-private-boundary','workflow-automation','state-machine','validation-routing','recovery','documentation-only')
$currentProfiles=@((Get-Content -Raw (Join-Path $workspace 'project.spec.json')|ConvertFrom-Json).validation_profiles.profile_id)
$currentResources=@('repo-path','build-output','android-package','headset','property-namespace','staging-namespace','bridge-port')
function Sha($p){(Get-FileHash -LiteralPath $p -Algorithm SHA256).Hash.ToLowerInvariant()}
function MapValue($kind,$value){
  switch("$kind/$value"){
    'category/activation' {return [ordered]@{legacy=$value;current='feature-activation';retained_as='legacy activation domain retained by unit tags, objective, and limitations'}}
    'category/compatibility' {return [ordered]@{legacy=$value;current='validation';retained_as='compatibility domain retained by unit tags, objective, and limitations'}}
    'category/documentation' {return [ordered]@{legacy=$value;current='documentation-only';retained_as='documentation domain retained by unit instruction surfaces and outputs'}}
    'profile/activation' {return [ordered]@{legacy=$value;current='source-only';retained_as='historical activation validation limitation'}}
    'profile/compatibility' {return [ordered]@{legacy=$value;current='source-only';retained_as='historical compatibility validation limitation'}}
    'resource/network-interface' {return [ordered]@{legacy=$value;current=$null;retained_as='historical observation-only interface limitation; no current lease semantics'}}
    default {throw "Unmapped legacy value: $kind/$value"}
  }
}
function BuildReceipt {
  $events=@(Get-Content (Join-Path $workspace 'iteration-events.jsonl')|ForEach-Object{$_|ConvertFrom-Json})
  $entries=@()
  foreach($file in Get-ChildItem (Join-Path $workspace 'iteration-units') -Filter *.json|Sort-Object Name){
    $u=Get-Content -Raw $file.FullName|ConvertFrom-Json
    $cats=@($u.change_categories|Where-Object{$currentCategories-notcontains $_}|Sort-Object -Unique)
    $profiles=@($u.validation.profile_id|Where-Object{$currentProfiles-notcontains $_}|Sort-Object -Unique)
    $resources=@();if($u.PSObject.Properties.Name-contains'resource_requirements'){$resources=@($u.resource_requirements.resource_kind|Where-Object{$currentResources-notcontains $_}|Sort-Object -Unique)}
    if($cats.Count+$profiles.Count+$resources.Count-eq0){continue}
    if(@('accepted','blocked')-notcontains $u.status){throw "Legacy nonterminal unit: $($u.unit_id)"}
    $terminal=if($u.status-eq'blocked'){@($events|Where-Object{$_.unit_id-eq$u.unit_id-and$_.event_type-eq'blocker'})[-1]}else{@($events|Where-Object{$_.unit_id-eq$u.unit_id-and$_.event_id-like'*-accepted-*'})[-1]}
    if($null-eq$terminal){throw "Missing terminal event: $($u.unit_id)"}
    $entries += [ordered]@{unit_id=$u.unit_id;unit_path="iteration-units/$($file.Name)";unit_sha256=Sha $file.FullName;terminal_status=$u.status;terminal_evidence=[ordered]@{event_id=$terminal.event_id;receipt_path=if(@($terminal.receipts).Count){[string]$terminal.receipts[0]}else{$null}};normalization=[ordered]@{change_categories=@($cats|ForEach-Object{MapValue category $_});validation_profiles=@($profiles|ForEach-Object{MapValue profile $_});resource_kinds=@($resources|ForEach-Object{MapValue resource $_})}}
  }
  if($entries.Count-ne14){throw "Expected 14 historical units, got $($entries.Count)"}
  return [ordered]@{'$schema'='https://github.com/MesmerPrism/rusty-morphospace-work-environment/schemas/historical-unit-adoption-receipt.schema.json';schema='rusty.morphospace.workflow.historical_unit_adoption_receipt.v1';receipt_id='rlsl-lslc-004q-historical-unit-adoption';project_id='rusty-lsl';source_workflow=[ordered]@{release='0.5.0+historical-adoption-v1';commit=$ownerCommit};units=$entries}
}
if($Generate){
  BuildReceipt|ConvertTo-Json -Depth 32|Set-Content -LiteralPath $receiptPath -Encoding utf8
  $state=Get-Content -Raw $statePath|ConvertFrom-Json
  $reference=[ordered]@{path='receipts/rlsl-lslc-004q-historical-unit-adoption.json';sha256=Sha $receiptPath}
  if($state.PSObject.Properties.Name-contains'historical_unit_adoption_receipts'){$state.historical_unit_adoption_receipts=@($reference)}else{$state|Add-Member -NotePropertyName historical_unit_adoption_receipts -NotePropertyValue @($reference)}
  $state|ConvertTo-Json -Depth 32|Set-Content -LiteralPath $statePath -Encoding utf8
}
if([string]::IsNullOrWhiteSpace($ownerRoot)){throw 'RLSL_WORK_ENVIRONMENT_ROOT must name the exact public owner materialization.'}
if((git -C $ownerRoot rev-parse HEAD).Trim()-ne$ownerCommit){throw 'Owner worktree commit drifted.'}
& (Join-Path $ownerRoot 'scripts/Test-WorkflowContracts.ps1') -RepoRoot $ownerRoot -WorkspaceRoot $workspace
if($LASTEXITCODE-ne0){exit $LASTEXITCODE}
$receipt=Get-Content -Raw $receiptPath|ConvertFrom-Json
if(@($receipt.units).Count-ne14){throw 'Historical adoption count drifted.'}
Write-Host 'LSLC-004Q exact historical workflow adoption passed.'
